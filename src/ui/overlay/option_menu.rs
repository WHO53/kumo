//! Dropdown like HTML <select> tags.

use std::ops::Mul;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cmp, mem};

use bitflags::bitflags;
use funq::MtQueueHandle;
use smithay_client_toolkit::seat::keyboard::Modifiers;

use crate::engine::EngineId;
use crate::ui::overlay::Popup;
use crate::ui::renderer::{Renderer, TextLayout, TextOptions, Texture, TextureBuilder};
use crate::ui::{SEPARATOR_HEIGHT, TOOLBAR_HEIGHT};
use crate::window::WindowId;
use crate::{gl, Position, Size, State};

// Option menu colors.
const FG: [f64; 3] = [1., 1., 1.];
const BG: [f64; 3] = [0.09, 0.09, 0.09];
const DISABLED_FG: [f64; 3] = [0.4, 0.4, 0.4];
const DISABLED_BG: [f64; 3] = BG;
const SELECTED_FG: [f64; 3] = [0.09, 0.09, 0.09];
const SELECTED_BG: [f64; 3] = [0.46, 0.16, 0.16];
const DESCRIPTION_FG: [f64; 3] = [0.75, 0.75, 0.75];
const BORDER_COLOR: [u8; 4] = [117, 42, 42, 255];

// Option menu item padding.
const X_PADDING: f64 = 15.;
const Y_PADDING: f64 = 10.;

// Border size at scale 1.
const BORDER_SIZE: u32 = 2;

/// Option item label font size.
const LABEL_FONT_SIZE: u8 = 16;
/// Option item description font size.
const DESCRIPTION_FONT_SIZE: u8 = 14;

/// Square of the maximum distance before touch input is considered a drag.
const MAX_TAP_DISTANCE: f64 = 400.;

/// Next option menu ID.
static NEXT_MENU_ID: AtomicUsize = AtomicUsize::new(0);

#[funq::callbacks(State)]
trait OptionMenuHandler {
    /// Indicate selection of an option item.
    fn option_menu_submit(&mut self, menu_id: OptionMenuId, index: usize);
}

impl OptionMenuHandler for State {
    fn option_menu_submit(&mut self, menu_id: OptionMenuId, index: usize) {
        let window = match self.windows.get_mut(&menu_id.window_id()) {
            Some(window) => window,
            None => return,
        };

        // Submit for the window if there's no engine ID attached.
        let engine_id = match menu_id.engine_id() {
            Some(engine_id) => engine_id,
            None => {
                window.submit_option_menu(menu_id, index);
                return;
            },
        };

        // Submit for the engine.
        let engine = match window.tabs_mut().get_mut(&engine_id) {
            Some(engine) => engine,
            None => return,
        };
        engine.submit_option_menu(menu_id, index);
        engine.close_option_menu(Some(menu_id));
    }
}

/// Option menu state.
pub struct OptionMenu {
    id: OptionMenuId,
    items: Vec<OptionMenuRenderItem>,
    selection_index: Option<usize>,

    queue: MtQueueHandle<State>,

    touch_state: TouchState,
    scroll_offset: f32,

    position: Position,
    max_height: u32,
    width: u32,
    scale: f64,

    borders: Borders,
    border: Option<Texture>,

    dirty: bool,
}

impl OptionMenu {
    pub fn new<I>(
        id: OptionMenuId,
        queue: MtQueueHandle<State>,
        position: Position,
        item_width: u32,
        max_size: Size,
        scale: f64,
        items: I,
    ) -> Self
    where
        I: Iterator<Item = OptionMenuItem>,
    {
        let mut menu = Self {
            position,
            queue,
            scale,
            id,
            borders: Borders::all(),
            width: item_width,
            selection_index: Default::default(),
            scroll_offset: Default::default(),
            touch_state: Default::default(),
            max_height: Default::default(),
            border: Default::default(),
            items: Default::default(),
            dirty: Default::default(),
        };

        let item_width = menu.item_width();
        menu.items = items
            .enumerate()
            .map(|(i, item)| {
                // Update selected item.
                if item.selected {
                    menu.selection_index = Some(i);
                }

                OptionMenuRenderItem::new(item, item_width, scale)
            })
            .collect();

        // Set initial size constraints.
        menu.set_size(max_size);

        menu
    }

    /// Get the popup's ID,
    pub fn id(&self) -> OptionMenuId {
        self.id
    }

    /// Move scroll position.
    pub fn scroll(&mut self, target: ScrollTarget) {
        let scroll_offset = match target {
            ScrollTarget::End => self.max_scroll_offset() as f32,
        };
        self.dirty = self.scroll_offset != scroll_offset;
        self.scroll_offset = scroll_offset;
    }

    /// Update popup borders.
    pub fn set_borders(&mut self, borders: Borders) {
        self.borders = borders;
    }

    fn item_width(&self) -> u32 {
        let border_widths = self.border_widths();
        self.width - border_widths.left - border_widths.right
    }

    /// Get index of item at the specified physical point.
    fn item_at(&self, mut position: Position<f64>) -> Option<usize> {
        // Apply border offsets.
        let borders = self.border_widths() * self.scale;
        position.y -= borders.top as f64;
        position.x -= borders.left as f64;

        // Apply current scroll offset.
        position.y += self.scroll_offset as f64;

        // Ignore points entirely outside the menu.
        let physical_width = self.width as f64 * self.scale - (borders.left - borders.right) as f64;
        if position.x < 0. || position.y < 0. || position.x >= physical_width {
            return None;
        }

        // Find item at the point's Y position.
        let mut y_end = 0.;
        for (i, item) in self.items.iter().enumerate() {
            y_end += item.height() as f64;
            if position.y < y_end {
                return Some(i);
            }
        }

        None
    }

    /// Clamp tabs view viewport offset.
    fn clamp_scroll_offset(&mut self) {
        let old_offset = self.scroll_offset;
        let max_offset = self.max_scroll_offset() as f32;
        self.scroll_offset = self.scroll_offset.clamp(0., max_offset);
        self.dirty |= old_offset != self.scroll_offset;
    }

    /// Get maximum tab scroll offset.
    fn max_scroll_offset(&self) -> u32 {
        let border_widths = self.border_widths() * self.scale;

        let max_height = (self.max_height as f64 * self.scale).round() as u32;
        let border_size = border_widths.top + border_widths.bottom;

        self.content_height().saturating_sub(max_height - border_size)
    }

    /// Get total option menu height in physical space.
    ///
    /// This is the height of the menu's content without maximum height
    /// constraints.
    fn content_height(&self) -> u32 {
        self.items.iter().map(|item| item.height() as u32).sum()
    }

    /// Get total option menu content height in logical space.
    ///
    /// See [`Self::total_height`] for more details.
    fn logical_content_height(&self) -> u32 {
        (self.content_height() as f64 / self.scale).round() as u32
    }

    /// Logical border widths.
    fn border_widths(&self) -> BorderWidths {
        BorderWidths {
            bottom: if self.borders.contains(Borders::BOTTOM) { BORDER_SIZE } else { 0 },
            right: if self.borders.contains(Borders::RIGHT) { BORDER_SIZE } else { 0 },
            left: if self.borders.contains(Borders::LEFT) { BORDER_SIZE } else { 0 },
            top: if self.borders.contains(Borders::TOP) { BORDER_SIZE } else { 0 },
        }
    }

    /// Logical height of the URI toolbar without separator.
    fn toolbar_height(&self) -> u32 {
        (TOOLBAR_HEIGHT - SEPARATOR_HEIGHT).round() as u32
    }
}

impl Popup for OptionMenu {
    fn dirty(&self) -> bool {
        self.dirty || self.items.iter().any(|item| item.dirty)
    }

    fn draw(&mut self, renderer: &Renderer) {
        self.dirty = false;

        // Ensure offset is correct in case size changed.
        self.clamp_scroll_offset();

        // Calculate physical menu dimensions.
        let mut position: Position<f32> = (self.position() * self.scale).into();
        let size = self.size() * self.scale;

        // Draw menu border.
        unsafe {
            let border = self.border.get_or_insert_with(|| Texture::new(&BORDER_COLOR, 1, 1));
            renderer.draw_texture_at(border, position, Some(size.into()));
        }

        // Scissor crop last element when it should only be partially visible.
        let borders = self.border_widths() * self.scale;
        let toolbar_height = (self.toolbar_height() as f64 * self.scale).round();
        let y = toolbar_height as i32 + borders.bottom as i32;
        let height = (self.max_height as f64 * self.scale).round() as i32 - borders.bottom as i32;
        unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            gl::Scissor(0, y, i32::MAX, height);
        }

        // Calculate menu position.
        position.x += borders.left as f32;
        position.y += borders.top as f32 - self.scroll_offset;

        // Draw each option menu entry.
        let max_height = (self.max_height as f32 * self.scale as f32).round();
        for (i, item) in self.items.iter_mut().enumerate() {
            // NOTE: This must be called on all textures to clear dirtiness flag.
            let selected = self.selection_index == Some(i);
            let texture = item.texture(selected);

            // Skip rendering out of bounds textures.
            if position.y + texture.height as f32 >= 0. && position.y < max_height {
                unsafe { renderer.draw_texture_at(texture, position, None) };
            }

            position.y += texture.height as f32;
        }

        // Disable scissoring again.
        unsafe { gl::Disable(gl::SCISSOR_TEST) };
    }

    fn position(&self) -> Position {
        let border_widths = self.border_widths();

        // Ensure popup is within display area.
        let max_height = self.max_height as i32;
        let height = self.logical_content_height() + border_widths.top + border_widths.bottom;
        let y_end = self.position.y + height as i32;
        let clamp_delta = cmp::max(y_end - max_height, 0);
        let y = cmp::max(self.position.y - clamp_delta, 0);

        Position::new(self.position.x, y)
    }

    fn set_size(&mut self, size: Size) {
        self.max_height = size.height - self.toolbar_height();
        self.dirty = true;
    }

    fn size(&self) -> Size {
        let border_widths = self.border_widths();
        let total_height = self.logical_content_height() + border_widths.top + border_widths.bottom;
        let height = cmp::min(self.max_height, total_height);

        Size::new(self.width, height)
    }

    fn set_scale(&mut self, scale: f64) {
        self.scale = scale;

        // Update option menu entries.
        let item_width = self.item_width();
        for item in &mut self.items {
            item.set_scale(scale);
            item.set_width(item_width);
        }
    }

    fn opaque_region(&self) -> Size {
        self.size()
    }

    fn touch_down(&mut self, _time: u32, id: i32, position: Position<f64>, _modifiers: Modifiers) {
        // Only accept a single touch point.
        if self.touch_state.slot.is_some() {
            return;
        }
        self.touch_state.slot = Some(id);

        // Keep track of touch position for release.
        let position = position * self.scale;
        self.touch_state.position = position;
        self.touch_state.start = position;

        // Reset touch action.
        self.touch_state.action = TouchAction::Tap;

        // Update selected item.
        let new_selected = self.item_at(position);
        if new_selected != self.selection_index {
            // Always clear currently selected item.
            if let Some(old_index) = self.selection_index {
                self.items[old_index].dirty = true;
            }

            // Select new item if there is one under the touch point.
            if let Some(new_index) = new_selected {
                self.items[new_index].dirty = true;
            }

            self.selection_index = new_selected;
        }
    }

    fn touch_motion(
        &mut self,
        _time: u32,
        id: i32,
        position: Position<f64>,
        _modifiers: Modifiers,
    ) {
        // Ignore all unknown touch points.
        if self.touch_state.slot != Some(id) {
            return;
        }

        // Keep track of touch position for release.
        let position = position * self.scale;
        let old_position = mem::replace(&mut self.touch_state.position, position);

        // Switch to dragging once tap distance limit is exceeded.
        let delta = self.touch_state.position - self.touch_state.start;
        if delta.x.powi(2) + delta.y.powi(2) > MAX_TAP_DISTANCE {
            self.touch_state.action = TouchAction::Drag;

            // Immediately start scrolling the menu.
            let old_offset = self.scroll_offset;
            self.scroll_offset += (old_position.y - self.touch_state.position.y) as f32;
            self.clamp_scroll_offset();
            self.dirty |= self.scroll_offset != old_offset;
        }
    }

    fn touch_up(&mut self, _time: u32, id: i32, _modifiers: Modifiers) {
        // Ignore all unknown touch points.
        if self.touch_state.slot != Some(id) {
            return;
        }
        self.touch_state.slot = None;

        if self.touch_state.action == TouchAction::Tap {
            if let Some(index) = self.item_at(self.touch_state.position) {
                self.queue.option_menu_submit(self.id, index);
            }
        }
    }
}

/// Entry in an option menu.
pub struct OptionMenuItem {
    /// Option menu text.
    pub label: String,
    /// Option menu detail text.
    pub description: String,
    /// Whether item is selectable.
    pub disabled: bool,
    /// Whether item is selected.
    pub selected: bool,
}

/// State for rendering an option menu entry.
struct OptionMenuRenderItem {
    /// Item texture cache.
    texture: Option<Texture>,
    dirty: bool,

    /// Desired logical texture width.
    width: u32,
    /// Render scale.
    scale: f64,

    /// Pango layout for main text.
    label_layout: TextLayout,
    /// Pange layout for description text.
    description_layout: Option<TextLayout>,
    /// Whether item is selectable.
    disabled: bool,
}

impl OptionMenuRenderItem {
    fn new(item: OptionMenuItem, item_width: u32, scale: f64) -> Self {
        // Create a new pango layout.
        let create_layout = |text: String, font_size: u8| {
            let layout = TextLayout::new(font_size, scale);
            layout.set_text(&text);
            layout.set_height(0);
            layout
        };

        let description_layout = (!item.description.is_empty())
            .then(|| create_layout(item.description, DESCRIPTION_FONT_SIZE));
        let label_layout = create_layout(item.label, LABEL_FONT_SIZE);

        OptionMenuRenderItem {
            description_layout,
            label_layout,
            scale,
            disabled: item.disabled,
            width: item_width,
            texture: None,
            dirty: true,
        }
    }

    fn texture(&mut self, selected: bool) -> &Texture {
        // Ensure texture is up to date.
        if mem::take(&mut self.dirty) {
            if let Some(texture) = self.texture.take() {
                texture.delete();
            }
            self.texture = Some(self.draw(selected));
        }

        self.texture.as_ref().unwrap()
    }

    fn draw(&self, selected: bool) -> Texture {
        // Determine item colors.
        let (fg, description_fg, bg) = if self.disabled {
            (DISABLED_FG, DISABLED_FG, DISABLED_BG)
        } else if selected {
            (SELECTED_FG, SELECTED_FG, SELECTED_BG)
        } else {
            (FG, DESCRIPTION_FG, BG)
        };

        // Calculate physical item size.
        let width = (self.width as f64 * self.scale).round() as i32;
        let physical_size = Size::new(width, self.height());

        // Initialize as opaque texture.
        let builder = TextureBuilder::new(physical_size);
        builder.clear(bg);

        // Configure text rendering options.
        let mut text_options = TextOptions::new();
        text_options.text_color(fg);

        // Calculate label text placement.
        let y_padding = (Y_PADDING * self.scale).round() as i32;
        let x_padding = (X_PADDING * self.scale).round() as i32;
        let label_height = self.label_layout.pixel_size().1 + y_padding;
        let label_width = physical_size.width - 2 * x_padding;
        text_options.position(Position::new(x_padding, 0).into());
        text_options.size(Size::new(label_width, label_height));

        // Render label text to texture.
        builder.rasterize(&self.label_layout, &text_options);

        // Render text description.
        if let Some(description_layout) = &self.description_layout {
            // Calculate description text placement.
            let description_height = description_layout.pixel_size().1 + y_padding;
            let description_size = Size::new(label_width, description_height);
            text_options.position(Position::new(x_padding, label_height - y_padding).into());
            text_options.size(description_size);

            // Render description to texture.
            text_options.text_color(description_fg);
            builder.rasterize(description_layout, &text_options);
        }

        builder.build()
    }

    /// Get the item's height.
    fn height(&self) -> i32 {
        let y_padding = (Y_PADDING * self.scale).round() as i32;
        let label_height = self.label_layout.pixel_size().1;
        let description_height = self.description_layout.as_ref().map_or(0, |l| l.pixel_size().1);
        label_height + description_height + y_padding
    }

    /// Update item width.
    fn set_width(&mut self, width: u32) {
        self.width = width;
        self.dirty = true;
    }

    /// Update item scale.
    fn set_scale(&mut self, scale: f64) {
        if let Some(layout) = &mut self.description_layout {
            layout.set_scale(scale);
        }
        self.label_layout.set_scale(scale);
        self.scale = scale;
        self.dirty = true;
    }
}

/// Unique identifier for an option menu.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OptionMenuId {
    window_id: WindowId,
    engine_id: Option<EngineId>,
    id: usize,
}

impl OptionMenuId {
    /// Create new ID for menu spawned by window.
    pub fn new(window_id: WindowId) -> Self {
        let id = NEXT_MENU_ID.fetch_add(1, Ordering::Relaxed);
        Self { id, window_id, engine_id: None }
    }

    /// Create new ID for menu spawned by engine.
    pub fn with_engine(engine_id: EngineId) -> Self {
        let id = NEXT_MENU_ID.fetch_add(1, Ordering::Relaxed);
        Self { id, window_id: engine_id.window_id(), engine_id: Some(engine_id) }
    }

    /// Get the menu's engine.
    pub fn engine_id(&self) -> Option<EngineId> {
        self.engine_id
    }

    /// Get the menu's window.
    pub fn window_id(&self) -> WindowId {
        self.window_id
    }
}

/// Touch event tracking.
#[derive(Default)]
struct TouchState {
    slot: Option<i32>,
    action: TouchAction,
    start: Position<f64>,
    position: Position<f64>,
}

/// Intention of a touch sequence.
#[derive(Default, Copy, Clone, PartialEq, Eq, Debug)]
enum TouchAction {
    #[default]
    Tap,
    Drag,
}

/// Target position for scrolling a menu.
pub enum ScrollTarget {
    End,
}

bitflags! {
    /// Popup borders.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Borders: u32 {
        const TOP    = 0b0001;
        const RIGHT  = 0b0010;
        const BOTTOM = 0b0100;
        const LEFT   = 0b1000;
    }
}

// Popup border widths.
#[derive(Debug)]
struct BorderWidths {
    bottom: u32,
    right: u32,
    left: u32,
    top: u32,
}

impl Mul<f64> for BorderWidths {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            bottom: (self.bottom as f64 * rhs).round() as u32,
            right: (self.right as f64 * rhs).round() as u32,
            left: (self.left as f64 * rhs).round() as u32,
            top: (self.top as f64 * rhs).round() as u32,
        }
    }
}
