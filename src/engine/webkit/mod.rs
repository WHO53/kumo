use std::any::Any;
use std::collections::HashMap;
use std::ffi::{self, CString};
use std::ptr;
use std::sync::Once;
use std::time::UNIX_EPOCH;

use funq::StQueueHandle;
use glib::object::ObjectExt;
use glutin::api::egl::Egl;
use glutin::display::{AsRawDisplay, Display, RawDisplay};
use smithay_client_toolkit::reexports::client::protocol::wl_buffer::WlBuffer;
use smithay_client_toolkit::reexports::client::{Connection, Proxy};
use smithay_client_toolkit::seat::keyboard::{Keysym, Modifiers};
use smithay_client_toolkit::seat::pointer::AxisScroll;
use wpe_backend_fdo_sys::{
    wpe_fdo_egl_exported_image, wpe_fdo_egl_exported_image_get_egl_image,
    wpe_fdo_egl_exported_image_get_height, wpe_fdo_egl_exported_image_get_width,
    wpe_input_axis_2d_event, wpe_input_axis_event,
    wpe_input_axis_event_type_wpe_input_axis_event_type_mask_2d,
    wpe_input_axis_event_type_wpe_input_axis_event_type_motion_smooth, wpe_input_keyboard_event,
    wpe_input_modifier_wpe_input_keyboard_modifier_alt,
    wpe_input_modifier_wpe_input_keyboard_modifier_control,
    wpe_input_modifier_wpe_input_keyboard_modifier_meta,
    wpe_input_modifier_wpe_input_keyboard_modifier_shift, wpe_input_pointer_event,
    wpe_input_pointer_event_type_wpe_input_pointer_event_type_button,
    wpe_input_pointer_event_type_wpe_input_pointer_event_type_motion, wpe_input_touch_event,
    wpe_input_touch_event_raw, wpe_input_touch_event_type,
    wpe_input_touch_event_type_wpe_input_touch_event_type_down,
    wpe_input_touch_event_type_wpe_input_touch_event_type_motion,
    wpe_input_touch_event_type_wpe_input_touch_event_type_up,
    wpe_view_activity_state_wpe_view_activity_state_focused,
    wpe_view_activity_state_wpe_view_activity_state_in_window,
    wpe_view_activity_state_wpe_view_activity_state_visible, wpe_view_backend_add_activity_state,
    wpe_view_backend_dispatch_axis_event, wpe_view_backend_dispatch_keyboard_event,
    wpe_view_backend_dispatch_pointer_event, wpe_view_backend_dispatch_set_device_scale_factor,
    wpe_view_backend_dispatch_set_size, wpe_view_backend_dispatch_touch_event,
    wpe_view_backend_exportable_fdo, wpe_view_backend_exportable_fdo_dispatch_frame_complete,
    wpe_view_backend_exportable_fdo_egl_client, wpe_view_backend_exportable_fdo_egl_create,
    wpe_view_backend_exportable_fdo_egl_dispatch_release_exported_image,
    wpe_view_backend_exportable_fdo_get_view_backend, wpe_view_backend_remove_activity_state,
};
use wpe_webkit::{
    Color, CookieAcceptPolicy, CookiePersistentStorage, NetworkSession, OptionMenu, WebView,
    WebViewBackend, WebViewExt,
};

use crate::engine::webkit::input_method_context::InputMethodContext;
use crate::engine::{Engine, EngineId, BG};
use crate::ui::overlay::option_menu::{OptionMenuId, OptionMenuItem};
use crate::wayland::protocols::BufferData;
use crate::window::TextInputChange;
use crate::{Position, Size, State};

mod input_method_context;

// Once for calling FDO initialization methods.
static FDO_INIT: Once = Once::new();

/// WebKit-specific errors.
#[derive(thiserror::Error, Debug)]
pub enum WebKitError {
    #[error("backend creation failed")]
    BackendCreation,
    #[error("could not load libWPEBackend-fdo-1.0.so, make sure it is installed")]
    FdoLibInit,
    #[error("failed to initialize fdo egl backend")]
    EglInit,
}

#[funq::callbacks(State, thread_local)]
trait WebKitHandler {
    /// Update the engine's underlying EGL image.
    fn set_egl_image(&mut self, engine_id: EngineId, image: *mut wpe_fdo_egl_exported_image);

    /// Update current URI for an engine.
    fn set_engine_uri(&mut self, engine_id: EngineId, uri: String);

    /// Update current title for an engine.
    fn set_engine_title(&mut self, engine_id: EngineId, title: String);

    /// Open dropdown popup.
    fn open_option_menu(
        &mut self,
        engine_id: EngineId,
        option_menu: OptionMenu,
        rect: (i32, i32, i32, i32),
    );

    /// Close dropdown popup.
    fn close_option_menu(&mut self, menu_id: OptionMenuId);
}

impl WebKitHandler for State {
    fn set_egl_image(&mut self, engine_id: EngineId, image: *mut wpe_fdo_egl_exported_image) {
        let window = match self.windows.get_mut(&engine_id.window_id()) {
            Some(window) => window,
            None => return,
        };
        let engine = match window.tabs_mut().get_mut(&engine_id) {
            Some(engine) => engine,
            None => return,
        };
        let webkit_engine = match engine.as_any().downcast_mut::<WebKitEngine>() {
            Some(webkit_engine) => webkit_engine,
            None => return,
        };

        // Request new image if submitted one is of incorrect size.
        unsafe {
            let width = wpe_fdo_egl_exported_image_get_width(image);
            let height = wpe_fdo_egl_exported_image_get_height(image);
            let desired_width =
                (webkit_engine.target_size.width as f32 * webkit_engine.scale).round() as u32;
            let desired_height =
                (webkit_engine.target_size.height as f32 * webkit_engine.scale).round() as u32;

            if desired_width != width || desired_height != height {
                webkit_engine.frame_done();
                wpe_view_backend_exportable_fdo_egl_dispatch_release_exported_image(
                    webkit_engine.exportable,
                    image,
                );
                return;
            }
        }

        // Update engine's WlBuffer.
        webkit_engine.import_image(&self.connection, &self.egl_display, image);

        // Offer new WlBuffer to window.
        if window.active_tab() == engine_id {
            window.unstall();
        }
    }

    fn set_engine_uri(&mut self, engine_id: EngineId, uri: String) {
        let window_id = engine_id.window_id();

        if let Some(window) = self.windows.get_mut(&window_id) {
            window.set_engine_uri(&self.history, engine_id, uri);
        }
    }

    fn set_engine_title(&mut self, engine_id: EngineId, title: String) {
        let window_id = engine_id.window_id();

        if let Some(window) = self.windows.get_mut(&window_id) {
            window.set_engine_title(&self.history, engine_id, title);
        }
    }

    fn open_option_menu(
        &mut self,
        engine_id: EngineId,
        option_menu: OptionMenu,
        (x, y, width, height): (i32, i32, i32, i32),
    ) {
        let window = match self.windows.get_mut(&engine_id.window_id()) {
            Some(window) => window,
            None => return,
        };
        let engine = match window.tabs_mut().get_mut(&engine_id) {
            Some(engine) => engine,
            None => return,
        };
        let webkit_engine = match engine.as_any().downcast_mut::<WebKitEngine>() {
            Some(webkit_engine) => webkit_engine,
            None => return,
        };

        // Get properties from WebKit menu items.
        let mut items = Vec::new();
        for i in 0..option_menu.n_items() {
            if let Some(mut item) = option_menu.item(i) {
                if let Some(label) = item.label() {
                    items.push(OptionMenuItem {
                        label: label.into(),
                        disabled: !item.is_enabled(),
                        selected: item.is_selected(),
                    });
                }
            }
        }

        // Get dimensions.
        let position = Position::new(x, y + height);
        let item_size = Size::new(width as u32, height as u32);

        // Hookup close callback.
        let menu_id = OptionMenuId::new(engine_id);
        let close_queue = self.queue.clone();
        option_menu.connect_close(move |_| close_queue.clone().close_option_menu(menu_id));

        // Update engine's active popup for close/activate handling.
        if let Some((_, option_menu)) = webkit_engine.option_menu.take() {
            option_menu.close();
        }
        webkit_engine.option_menu = Some((menu_id, option_menu));

        // Show the popup.
        window.open_option_menu(menu_id, position, item_size, items.into_iter());
    }

    fn close_option_menu(&mut self, menu_id: OptionMenuId) {
        if let Some(window) = self.windows.get_mut(&menu_id.window_id()) {
            window.close_option_menu(menu_id);
        }
    }
}

/// WebKit browser engine.
pub struct WebKitEngine {
    input_method_context: InputMethodContext,
    backend: WebViewBackend,
    web_view: WebView,

    exportable: *mut wpe_view_backend_exportable_fdo,
    image: *mut wpe_fdo_egl_exported_image,
    buffer: Option<WlBuffer>,

    target_size: Size,
    buffer_size: Size,
    scale: f32,

    // Mouse pointer state.
    pointer_button: u32,
    pointer_state: u32,

    egl: &'static Egl,

    id: EngineId,

    option_menu: Option<(OptionMenuId, OptionMenu)>,

    dirty: bool,
}

impl Drop for WebKitEngine {
    fn drop(&mut self) {
        unsafe {
            // Free EGL image.
            if !self.image.is_null() {
                wpe_view_backend_exportable_fdo_egl_dispatch_release_exported_image(
                    self.exportable,
                    self.image,
                );
            }
        }
    }
}

impl WebKitEngine {
    pub fn new(
        display: &Display,
        queue: StQueueHandle<State>,
        engine_id: EngineId,
        size: Size,
        scale: f64,
    ) -> Result<Self, WebKitError> {
        // Ensure FDO is initialized.
        let mut result = Ok(());
        FDO_INIT.call_once(|| result = Self::init_fdo(display));
        result?;

        // Create web view backend.
        let backend_queue = queue.clone();
        let (mut backend, exportable) = unsafe {
            // Create EGL FDO backend.
            let exportable = create_exportable_backend(engine_id, backend_queue, size);
            let egl_backend = wpe_view_backend_exportable_fdo_get_view_backend(exportable);
            if egl_backend.is_null() {
                return Err(WebKitError::BackendCreation);
            }

            (WebViewBackend::new(egl_backend), exportable)
        };

        // Create web view with initial blank page.
        let network_session = xdg_network_session().unwrap_or_else(NetworkSession::new_ephemeral);
        let web_view =
            WebView::builder().network_session(&network_session).backend(&backend).build();
        web_view.load_uri("about:blank");

        // Set browser background color.
        let mut color = Color::new(BG[0], BG[1], BG[2], 1.);
        web_view.set_background_color(&mut color);

        // Notify UI about URI and title changes.
        let uri_queue = queue.clone();
        web_view.connect_uri_notify(move |web_view| {
            let uri = web_view.uri().unwrap_or_default().to_string();
            uri_queue.clone().set_engine_uri(engine_id, uri);
        });
        let title_queue = queue.clone();
        web_view.connect_title_notify(move |web_view| {
            let title = web_view.title().unwrap_or_default().to_string();
            title_queue.clone().set_engine_title(engine_id, title);
        });

        web_view.connect_show_option_menu(move |_, menu, rect| {
            queue.clone().open_option_menu(engine_id, menu.clone(), rect.geometry());
            true
        });

        // Setup input handler.
        let input_method_context = InputMethodContext::new();
        web_view.set_input_method_context(Some(&input_method_context));

        // Set initial activity state.
        let state = wpe_view_activity_state_wpe_view_activity_state_visible
            | wpe_view_activity_state_wpe_view_activity_state_in_window
            | wpe_view_activity_state_wpe_view_activity_state_focused;
        unsafe {
            wpe_view_backend_add_activity_state(backend.wpe_backend(), state);
        }

        // Get access to the OpenGL API.
        let Display::Egl(egl_display) = display;
        let egl = egl_display.egl();

        let mut engine = Self {
            input_method_context,
            exportable,
            web_view,
            backend,
            target_size: size,
            egl,
            image: ptr::null_mut(),
            id: engine_id,
            scale: 1.0,
            pointer_button: Default::default(),
            pointer_state: Default::default(),
            buffer_size: Default::default(),
            option_menu: Default::default(),
            buffer: Default::default(),
            dirty: Default::default(),
        };

        // Update engine scale.
        engine.set_scale(scale);

        Ok(engine)
    }

    /// Import a new EGLImage as WlBuffer.
    fn import_image(
        &mut self,
        connection: &Connection,
        egl_display: &Display,
        image: *mut wpe_fdo_egl_exported_image,
    ) {
        // Require redraw.
        self.dirty = true;

        // Free previous image.
        if !self.image.is_null() {
            unsafe {
                wpe_view_backend_exportable_fdo_egl_dispatch_release_exported_image(
                    self.exportable,
                    self.image,
                );
            }
        }

        self.buffer_size = self.target_size * self.scale as f64;
        self.image = image;

        let RawDisplay::Egl(raw_display) = egl_display.raw_display();

        // Convert EGLImage to WlBuffer.
        let object_id = unsafe {
            let egl_image = wpe_fdo_egl_exported_image_get_egl_image(self.image);
            let raw_wl_buffer = self.egl.CreateWaylandBufferFromImageWL(raw_display, egl_image);
            let data = BufferData::new(connection.clone());
            connection.backend().manage_object(WlBuffer::interface(), raw_wl_buffer.cast(), data)
        };
        self.buffer = Some(WlBuffer::from_id(connection, object_id).unwrap());
    }

    /// Initialize the WPEBackend-fdo library.
    fn init_fdo(display: &Display) -> Result<(), WebKitError> {
        unsafe {
            let RawDisplay::Egl(display) = display.raw_display();

            let backend_lib = CString::new("libWPEBackend-fdo-1.0.so").unwrap();
            if !wpe_backend_fdo_sys::wpe_loader_init(backend_lib.as_ptr()) {
                return Err(WebKitError::FdoLibInit);
            }

            if !wpe_backend_fdo_sys::wpe_fdo_initialize_for_egl_display(display as *mut _) {
                return Err(WebKitError::EglInit);
            }

            Ok(())
        }
    }

    /// Emit a touch input event.
    fn touch_event(
        &mut self,
        touch_points: &[wpe_input_touch_event_raw],
        time: u32,
        id: i32,
        modifiers: Modifiers,
        type_: wpe_input_touch_event_type,
    ) {
        let mut event = wpe_input_touch_event {
            type_,
            time,
            id,
            touchpoints_length: touch_points.len() as u64,
            modifiers: wpe_modifiers(modifiers),
            touchpoints: touch_points.as_ptr(),
        };

        unsafe {
            let wpe_backend = self.backend.wpe_backend();
            wpe_view_backend_dispatch_touch_event(wpe_backend, &mut event);
        }
    }

    /// Update engine focus.
    fn set_focused(&mut self, focused: bool) {
        // Force text-input update.
        self.input_method_context.mark_text_input_dirty();

        let state = wpe_view_activity_state_wpe_view_activity_state_focused;
        unsafe {
            let backend = self.backend.wpe_backend();
            if focused {
                wpe_view_backend_add_activity_state(backend, state);
            } else {
                wpe_view_backend_remove_activity_state(backend, state);
            }
        }
    }
}

impl Engine for WebKitEngine {
    fn id(&self) -> EngineId {
        self.id
    }

    fn wl_buffer(&self) -> Option<&WlBuffer> {
        self.buffer.as_ref()
    }

    fn dirty(&self) -> bool {
        self.dirty
    }

    fn frame_done(&mut self) {
        self.dirty = false;

        unsafe {
            wpe_view_backend_exportable_fdo_dispatch_frame_complete(self.exportable);
        }
    }

    fn set_size(&mut self, size: Size) {
        self.target_size = size;

        unsafe {
            let wpe_backend = self.backend.wpe_backend();
            wpe_view_backend_dispatch_set_size(wpe_backend, size.width, size.height);
        }
    }

    fn buffer_size(&self) -> Size {
        self.buffer_size
    }

    fn set_scale(&mut self, scale: f64) {
        // Clamp scale to WebKit's limits.
        //
        // https://webplatformforembedded.github.io/libwpe/view-backend.html#wpe_view_backend_dispatch_set_device_scale_factor
        self.scale = scale.clamp(0.05, 5.0) as f32;

        unsafe {
            let wpe_backend = self.backend.wpe_backend();
            wpe_view_backend_dispatch_set_device_scale_factor(wpe_backend, self.scale);
        }
    }

    fn press_key(&mut self, raw: u32, keysym: Keysym, modifiers: Modifiers) {
        let mut event = wpe_keyboard_event(raw, keysym, modifiers, true);
        unsafe {
            let wpe_backend = self.backend.wpe_backend();
            wpe_view_backend_dispatch_keyboard_event(wpe_backend, &mut event);
        }
    }

    fn release_key(&mut self, raw: u32, keysym: Keysym, modifiers: Modifiers) {
        let mut event = wpe_keyboard_event(raw, keysym, modifiers, false);
        unsafe {
            let wpe_backend = self.backend.wpe_backend();
            wpe_view_backend_dispatch_keyboard_event(wpe_backend, &mut event);
        }
    }

    fn pointer_axis(
        &mut self,
        time: u32,
        position: Position<f64>,
        horizontal: AxisScroll,
        vertical: AxisScroll,
        modifiers: Modifiers,
    ) {
        let type_ = wpe_input_axis_event_type_wpe_input_axis_event_type_motion_smooth
            | wpe_input_axis_event_type_wpe_input_axis_event_type_mask_2d;

        let mut axis_event = wpe_input_axis_2d_event {
            base: wpe_input_axis_event {
                type_,
                time,
                x: (position.x * self.scale as f64).round() as i32,
                y: (position.y * self.scale as f64).round() as i32,
                modifiers: wpe_modifiers(modifiers),
                axis: 0,
                value: 0,
            },
            x_axis: horizontal.absolute,
            y_axis: -vertical.absolute,
        };

        unsafe {
            let wpe_backend = self.backend.wpe_backend();
            wpe_view_backend_dispatch_axis_event(wpe_backend, &mut axis_event.base);
        }
    }

    fn pointer_button(
        &mut self,
        time: u32,
        position: Position<f64>,
        button: u32,
        state: u32,
        modifiers: Modifiers,
    ) {
        self.set_focused(true);

        self.pointer_button = button;
        self.pointer_state = state;

        let mut event = wpe_input_pointer_event {
            button: button - 271,
            state,
            time,
            type_: wpe_input_pointer_event_type_wpe_input_pointer_event_type_button,
            x: (position.x * self.scale as f64).round() as i32,
            y: (position.y * self.scale as f64).round() as i32,
            modifiers: wpe_modifiers(modifiers),
        };

        unsafe {
            let wpe_backend = self.backend.wpe_backend();
            wpe_view_backend_dispatch_pointer_event(wpe_backend, &mut event);
        }
    }

    fn pointer_motion(&mut self, time: u32, position: Position<f64>, modifiers: Modifiers) {
        let button = if self.pointer_state == 0 { 0 } else { self.pointer_button - 271 };

        let mut event = wpe_input_pointer_event {
            button,
            time,
            type_: wpe_input_pointer_event_type_wpe_input_pointer_event_type_motion,
            x: (position.x * self.scale as f64).round() as i32,
            y: (position.y * self.scale as f64).round() as i32,
            modifiers: wpe_modifiers(modifiers),
            state: self.pointer_state,
        };

        unsafe {
            let wpe_backend = self.backend.wpe_backend();
            wpe_view_backend_dispatch_pointer_event(wpe_backend, &mut event);
        }
    }

    fn touch_down(
        &mut self,
        touch_points: &HashMap<i32, Position<f64>>,
        time: u32,
        id: i32,
        modifiers: Modifiers,
    ) {
        self.set_focused(true);

        let event_type = wpe_input_touch_event_type_wpe_input_touch_event_type_down;
        let touch_points = wpe_touch_points(touch_points, self.scale, time, id, event_type);
        self.touch_event(&touch_points, time, id, modifiers, event_type);
    }

    fn touch_up(
        &mut self,
        touch_points: &HashMap<i32, Position<f64>>,
        time: u32,
        id: i32,
        modifiers: Modifiers,
    ) {
        let event_type = wpe_input_touch_event_type_wpe_input_touch_event_type_up;
        let touch_points = wpe_touch_points(touch_points, self.scale, time, id, event_type);
        self.touch_event(&touch_points, time, id, modifiers, event_type);
    }

    fn touch_motion(
        &mut self,
        touch_points: &HashMap<i32, Position<f64>>,
        time: u32,
        id: i32,
        modifiers: Modifiers,
    ) {
        let event_type = wpe_input_touch_event_type_wpe_input_touch_event_type_motion;
        let touch_points = wpe_touch_points(touch_points, self.scale, time, id, event_type);
        self.touch_event(&touch_points, time, id, modifiers, event_type);
    }

    fn load_uri(&self, uri: &str) {
        self.web_view.load_uri(uri);
    }

    fn load_prev(&self) {
        self.web_view.go_back();
    }

    fn uri(&self) -> String {
        self.web_view.uri().unwrap_or_default().into()
    }

    fn title(&self) -> String {
        self.web_view.title().unwrap_or_default().into()
    }

    fn text_input_state(&self) -> TextInputChange {
        self.input_method_context.text_input_state()
    }

    fn delete_surrounding_text(&mut self, before_length: u32, after_length: u32) {
        self.input_method_context
            .emit_by_name::<()>("delete-surrounding", &[&before_length, &after_length]);
    }

    fn commit_string(&mut self, text: String) {
        self.input_method_context.emit_by_name::<()>("committed", &[&text]);
    }

    fn preedit_string(&mut self, _text: String, _cursor_begin: i32, _cursor_end: i32) {
        // NOTE: WebKit supports signaling preedit start/change/finish, but
        // doesn't support forwarding the preedit text itself.
    }

    fn clear_focus(&mut self) {
        self.set_focused(false);
    }

    fn option_menu_submit(&mut self, menu_id: OptionMenuId, index: usize) {
        if let Some((id, menu)) = &self.option_menu {
            if *id == menu_id {
                menu.activate_item(index as u32);
            }
        }
    }

    fn option_menu_close(&mut self, menu_id: Option<OptionMenuId>) {
        if let Some((id, menu)) = &self.option_menu {
            if menu_id.map_or(true, |menu_id| *id == menu_id) {
                menu.close();
                self.option_menu = None;
            }
        }
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

/// Construct WPE keyboard event from its components.
fn wpe_keyboard_event(
    raw: u32,
    keysym: Keysym,
    modifiers: Modifiers,
    pressed: bool,
) -> wpe_input_keyboard_event {
    // Get system time in seconds.
    let elapsed = UNIX_EPOCH.elapsed().unwrap_or_default();
    let time = elapsed.as_secs() as u32;

    wpe_input_keyboard_event {
        pressed,
        time,
        modifiers: wpe_modifiers(modifiers),
        hardware_key_code: raw,
        key_code: keysym.raw(),
    }
}

/// Convert Wayland modifiers to WPE modifiers.
fn wpe_modifiers(modifiers: Modifiers) -> u32 {
    let mut wpe_modifiers = 0;
    if modifiers.ctrl {
        wpe_modifiers += wpe_input_modifier_wpe_input_keyboard_modifier_control;
    }
    if modifiers.shift {
        wpe_modifiers += wpe_input_modifier_wpe_input_keyboard_modifier_shift;
    }
    if modifiers.alt {
        wpe_modifiers += wpe_input_modifier_wpe_input_keyboard_modifier_alt;
    }
    if modifiers.logo {
        wpe_modifiers += wpe_input_modifier_wpe_input_keyboard_modifier_meta;
    }
    wpe_modifiers
}

/// Convert touch points to WPE touch events.
fn wpe_touch_points(
    touch_points: &HashMap<i32, Position<f64>>,
    scale: f32,
    time: u32,
    main_id: i32,
    main_type: wpe_input_touch_event_type,
) -> Vec<wpe_input_touch_event_raw> {
    touch_points
        .iter()
        .map(|(&point_id, Position { x, y })| {
            // Pretend all existing touch points just moved in place.
            let type_ = if main_id == point_id {
                main_type
            } else {
                wpe_input_touch_event_type_wpe_input_touch_event_type_motion
            };

            let x = (x * scale as f64).round() as i32;
            let y = (y * scale as f64).round() as i32;

            wpe_input_touch_event_raw { type_, time, id: point_id, x, y }
        })
        .collect()
}

/// Shared state leaked to FDO backend callbacks.
struct ExportableSharedState {
    queue: StQueueHandle<State>,
    engine_id: EngineId,
}

/// Create the exportable FDO EGL backend.
unsafe fn create_exportable_backend(
    engine_id: EngineId,
    queue: StQueueHandle<State>,
    size: Size,
) -> *mut wpe_view_backend_exportable_fdo {
    let client = wpe_view_backend_exportable_fdo_egl_client {
        export_fdo_egl_image: Some(on_egl_image_export),
        export_shm_buffer: None,
        export_egl_image: None,
        _wpe_reserved0: None,
        _wpe_reserved1: None,
    };

    let client = Box::into_raw(Box::new(client));
    let state = Box::into_raw(Box::new(ExportableSharedState { engine_id, queue }));
    wpe_view_backend_exportable_fdo_egl_create(client, state.cast(), size.width, size.height)
}

/// Handle EGL backend image export.
unsafe extern "C" fn on_egl_image_export(
    data: *mut ffi::c_void,
    image: *mut wpe_fdo_egl_exported_image,
) {
    let state = data as *mut ExportableSharedState;
    let state = match state.as_mut() {
        Some(state) => state,
        None => return,
    };

    state.queue.set_egl_image(state.engine_id, image);
}

/// Get WebKit network session using XDG-based backing storage.
fn xdg_network_session() -> Option<NetworkSession> {
    // Create the network session using kumo-suffixed XDG directories.
    let data_dir = dirs::data_dir()?.join("kumo/default");
    let cache_dir = dirs::cache_dir()?.join("kumo/default");
    let network_session = NetworkSession::new(Some(data_dir.to_str()?), Some(cache_dir.to_str()?));

    // Setup SQLite cookie storage in xdg data dir.
    let cookie_manager = network_session.cookie_manager()?;
    let cookies_path = data_dir.join("cookies.sqlite");
    cookie_manager.set_persistent_storage(cookies_path.to_str()?, CookiePersistentStorage::Sqlite);

    // Prohibit third-party cookies.
    cookie_manager.set_accept_policy(CookieAcceptPolicy::NoThirdParty);

    Some(network_session)
}
