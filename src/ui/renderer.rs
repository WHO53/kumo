//! OpenGL UI rendering.

use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use std::ops::{Deref, Range};
use std::ptr::NonNull;
use std::{cmp, mem, ptr};

use glutin::config::{Api, ConfigTemplateBuilder};
use glutin::context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext, Version};
use glutin::display::Display;
use glutin::prelude::*;
use glutin::surface::{Surface, SurfaceAttributesBuilder, SwapInterval, WindowSurface};
use pangocairo::cairo::{Context, Format, ImageSurface};
use pangocairo::pango::{
    AttrColor, AttrInt, AttrList, EllipsizeMode, FontDescription, Layout, Underline,
    SCALE as PANGO_SCALE,
};
use raw_window_handle::{RawWindowHandle, WaylandWindowHandle};
use smithay_client_toolkit::reexports::client::protocol::wl_surface::WlSurface;
use smithay_client_toolkit::reexports::client::Proxy;

use crate::gl::types::{GLfloat, GLint, GLuint};
use crate::{gl, Position, Size};

// OpenGL shader programs.
const VERTEX_SHADER: &str = include_str!("../../shaders/vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("../../shaders/fragment.glsl");

// Colors for text selection.
const SELECTION_BG: [u16; 3] = [29952, 10752, 10752];
const SELECTION_FG: [u16; 3] = [0; 3];

// Selection caret height in pixels at scale 1.
const CARET_SIZE: f64 = 5.;

/// OpenGL renderer.
#[derive(Debug)]
pub struct Renderer {
    sized: Option<SizedRenderer>,
    surface: WlSurface,
    display: Display,
}

impl Renderer {
    /// Initialize a new renderer.
    pub fn new(display: Display, surface: WlSurface) -> Self {
        // Setup OpenGL symbol loader.
        gl::load_with(|symbol| {
            let symbol = CString::new(symbol).unwrap();
            display.get_proc_address(symbol.as_c_str()).cast()
        });

        Renderer { surface, display, sized: Default::default() }
    }

    /// Perform drawing with this renderer.
    pub fn draw<F: FnOnce(&Renderer)>(&mut self, size: Size, fun: F) {
        self.sized(size).make_current();

        // Resize OpenGL viewport.
        //
        // This isn't done in `Self::resize` since the renderer must be current.
        unsafe { gl::Viewport(0, 0, size.width as i32, size.height as i32) };

        fun(self);

        unsafe { gl::Flush() };

        self.sized(size).swap_buffers();
    }

    /// Get render state requiring a size.
    fn sized(&mut self, size: Size) -> &SizedRenderer {
        // Initialize or resize sized state.
        match &mut self.sized {
            // Resize renderer.
            Some(sized) => sized.resize(size),
            // Create sized state.
            None => {
                self.sized = Some(SizedRenderer::new(&self.display, &self.surface, size));
            },
        }

        self.sized.as_ref().unwrap()
    }

    /// Render texture at a position in viewport-coordinates.
    ///
    /// Specifying a `size` will automatically scale the texture to render at
    /// the desired size. Otherwise the texture's size will be used instead.
    pub unsafe fn draw_texture_at(
        &self,
        texture: &Texture,
        mut position: Position<f32>,
        size: impl Into<Option<Size<f32>>>,
    ) {
        // Fail before renderer initialization.
        //
        // The sized state should always be initialized since it only makes sense to
        // call this function within `Self::draw`'s closure.
        let sized = match &self.sized {
            Some(sized) => sized,
            None => unreachable!(),
        };

        let (width, height) = match size.into() {
            Some(Size { width, height }) => (width, height),
            None => (texture.width as f32, texture.height as f32),
        };

        // Matrix transforming vertex positions to desired size.
        let size: Size<f32> = sized.size.into();
        let x_scale = width / size.width;
        let y_scale = height / size.height;
        let matrix = [x_scale, 0., 0., y_scale];
        gl::UniformMatrix2fv(sized.uniform_matrix, 1, gl::FALSE, matrix.as_ptr());

        // Set texture position offset.
        position.x /= size.width / 2.;
        position.y /= size.height / 2.;
        gl::Uniform2fv(sized.uniform_position, 1, [position.x, -position.y].as_ptr());

        gl::BindTexture(gl::TEXTURE_2D, texture.id);

        gl::DrawArrays(gl::TRIANGLES, 0, 6);
    }
}

/// Render state requiring known size.
///
/// This state is initialized on-demand, to avoid Mesa's issue with resizing
/// before the first draw.
#[derive(Debug)]
struct SizedRenderer {
    uniform_position: GLint,
    uniform_matrix: GLint,

    egl_surface: Surface<WindowSurface>,
    egl_context: PossiblyCurrentContext,

    size: Size,
}

impl SizedRenderer {
    /// Create sized renderer state.
    fn new(display: &Display, surface: &WlSurface, size: Size) -> Self {
        // Create EGL surface and context and make it current.
        let (egl_surface, egl_context) = Self::create_surface(display, surface, size);

        // Setup OpenGL program.
        let (uniform_position, uniform_matrix) = Self::create_program();

        Self { uniform_position, uniform_matrix, egl_surface, egl_context, size }
    }

    /// Resize the renderer.
    fn resize(&mut self, size: Size) {
        if self.size == size {
            return;
        }

        // Resize EGL texture.
        self.egl_surface.resize(
            &self.egl_context,
            NonZeroU32::new(size.width).unwrap(),
            NonZeroU32::new(size.height).unwrap(),
        );

        self.size = size;
    }

    /// Make EGL surface current.
    fn make_current(&self) {
        self.egl_context.make_current(&self.egl_surface).unwrap();
    }

    /// Perform OpenGL buffer swap.
    fn swap_buffers(&self) {
        self.egl_surface.swap_buffers(&self.egl_context).unwrap();
    }

    /// Create a new EGL surface.
    fn create_surface(
        display: &Display,
        surface: &WlSurface,
        size: Size,
    ) -> (Surface<WindowSurface>, PossiblyCurrentContext) {
        assert!(size.width > 0 && size.height > 0);

        // Create EGL config.
        let config_template = ConfigTemplateBuilder::new().with_api(Api::GLES2).build();
        let egl_config = unsafe {
            display
                .find_configs(config_template)
                .ok()
                .and_then(|mut configs| configs.next())
                .unwrap()
        };

        // Create EGL context.
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(Some(Version::new(2, 0))))
            .build(None);
        let egl_context =
            unsafe { display.create_context(&egl_config, &context_attributes).unwrap() };
        let egl_context = egl_context.treat_as_possibly_current();

        let surface = NonNull::new(surface.id().as_ptr().cast()).unwrap();
        let raw_window_handle = WaylandWindowHandle::new(surface);
        let raw_window_handle = RawWindowHandle::Wayland(raw_window_handle);
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_window_handle,
            NonZeroU32::new(size.width).unwrap(),
            NonZeroU32::new(size.height).unwrap(),
        );

        let egl_surface =
            unsafe { display.create_window_surface(&egl_config, &surface_attributes).unwrap() };

        // Ensure rendering never blocks.
        egl_context.make_current(&egl_surface).unwrap();
        egl_surface.set_swap_interval(&egl_context, SwapInterval::DontWait).unwrap();

        (egl_surface, egl_context)
    }

    /// Create the OpenGL program.
    fn create_program() -> (GLint, GLint) {
        unsafe {
            // Create vertex shader.
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                [VERTEX_SHADER.as_ptr()].as_ptr() as *const _,
                &(VERTEX_SHADER.len() as i32) as *const _,
            );
            gl::CompileShader(vertex_shader);

            // Create fragment shader.
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                [FRAGMENT_SHADER.as_ptr()].as_ptr() as *const _,
                &(FRAGMENT_SHADER.len() as i32) as *const _,
            );
            gl::CompileShader(fragment_shader);

            // Create shader program.
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);
            gl::UseProgram(program);

            // Generate VBO.
            let mut vbo = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Fill VBO with vertex positions.
            #[rustfmt::skip]
            let vertices: [GLfloat; 12] = [
                -1.0,  1.0, // Top-left
                -1.0, -1.0, // Bottom-left
                 1.0, -1.0, // Bottom-right

                -1.0,  1.0, // Top-left
                 1.0, -1.0, // Bottom-right
                 1.0,  1.0, // Top-right
            ];
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (mem::size_of::<GLfloat>() * vertices.len()) as isize,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Define VBO layout.
            let name = CStr::from_bytes_with_nul(b"aVertexPosition\0").unwrap();
            let location = gl::GetAttribLocation(program, name.as_ptr()) as GLuint;
            gl::VertexAttribPointer(
                location,
                2,
                gl::FLOAT,
                gl::FALSE,
                2 * mem::size_of::<GLfloat>() as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Get uniform locations.
            let name = CStr::from_bytes_with_nul(b"uPosition\0").unwrap();
            let uniform_position = gl::GetUniformLocation(program, name.as_ptr());
            let name = CStr::from_bytes_with_nul(b"uMatrix\0").unwrap();
            let uniform_matrix = gl::GetUniformLocation(program, name.as_ptr());

            (uniform_position, uniform_matrix)
        }
    }
}

/// OpenGL texture.
#[derive(Debug)]
pub struct Texture {
    id: u32,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    /// Load a buffer as texture into OpenGL.
    pub fn new(buffer: &[u8], width: usize, height: usize) -> Self {
        assert!(buffer.len() == width * height * 4);

        unsafe {
            let mut id = 0;
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                buffer.as_ptr() as *const _,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            Self { id, width, height }
        }
    }

    /// Delete this texture.
    ///
    /// Since texture ID are context-specific, the context must be bound when
    /// calling this function.
    pub fn delete(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}

/// Cairo-based graphics rendering.
pub struct TextureBuilder {
    image_surface: ImageSurface,
    context: Context,
    size: Size<i32>,
}

impl TextureBuilder {
    pub fn new(size: Size<i32>) -> Self {
        let image_surface = ImageSurface::create(Format::ARgb32, size.width, size.height).unwrap();
        let context = Context::new(&image_surface).unwrap();

        Self { image_surface, context, size }
    }

    /// Fill entire buffer with a single color.
    pub fn clear(&self, color: [f64; 3]) {
        self.context.set_source_rgb(color[0], color[1], color[2]);
        self.context.paint().unwrap();
    }

    /// Draw text within the specified bounds.
    #[cfg_attr(feature = "profiling", profiling::function)]
    pub fn rasterize(&self, layout: &TextLayout, text_options: &TextOptions) {
        // Limit text size to builder limits.
        let position = text_options.position;
        let size = match text_options.size {
            Some(mut size) => {
                size.width = cmp::min(size.width, self.size.width - position.x.round() as i32);
                size.height = cmp::min(size.height, self.size.height - position.y.round() as i32);
                size
            },
            None => {
                let width = self.size.width - position.x.round() as i32;
                let height = self.size.height - position.y.round() as i32;
                Size::new(width, height)
            },
        };

        // Truncate text beyond specified bounds.
        if text_options.ellipsize {
            layout.set_width(size.width * PANGO_SCALE);
            layout.set_ellipsize(EllipsizeMode::End);
            layout.set_height(0);
        }

        // Calculate text position.
        let (_, text_height) = layout.pixel_size();
        let text_y = position.y + size.height as f64 / 2. - text_height as f64 / 2.;

        // Handle text selection.
        let color = text_options.text_color;
        if let Some(selection) = &text_options.selection {
            // Set fg/bg colors of selected text.

            let text_attributes = AttrList::new();

            let mut bg_attr =
                AttrColor::new_background(SELECTION_BG[0], SELECTION_BG[1], SELECTION_BG[2]);
            bg_attr.set_start_index(selection.start as u32);
            bg_attr.set_end_index(selection.end as u32);
            text_attributes.insert(bg_attr);

            let mut fg_attr =
                AttrColor::new_foreground(SELECTION_FG[0], SELECTION_FG[1], SELECTION_FG[2]);
            fg_attr.set_start_index(selection.start as u32);
            fg_attr.set_end_index(selection.end as u32);
            text_attributes.insert(fg_attr);

            layout.set_attributes(Some(&text_attributes));

            // Draw selection carets.
            let draw_caret = |index| {
                let (selection_cursor, _) = layout.cursor_pos(index);
                let caret_x = position.x + selection_cursor.x() as f64 / PANGO_SCALE as f64;
                let caret_size = CARET_SIZE * layout.scale;
                self.context.move_to(caret_x, text_y);
                self.context.line_to(caret_x + caret_size, text_y - caret_size);
                self.context.line_to(caret_x - caret_size, text_y - caret_size);
                self.context.set_source_rgb(color[0], color[1], color[2]);
                self.context.fill().unwrap();
            };
            draw_caret(selection.start);
            draw_caret(selection.end);
        }

        // Temporarily insert preedit and autocomplete text.
        let mut text_without_virtual = None;
        let has_preedit = !text_options.preedit.0.is_empty();
        let has_autocomplete = !text_options.autocomplete.is_empty();
        if has_preedit || has_autocomplete {
            // Store text before insertion.
            let mut virtual_text = layout.text().to_string();
            text_without_virtual = Some(virtual_text.clone());

            if has_preedit {
                // Insert preedit text.
                let preedit_start = text_options.cursor_pos as usize;
                let preedit_end = preedit_start + text_options.preedit.0.len();
                virtual_text.insert_str(preedit_start, &text_options.preedit.0);

                // Add underline below preedit text.
                let attributes = layout.attributes().unwrap_or_default();
                let mut ul_attr = AttrInt::new_underline(Underline::Single);
                ul_attr.set_start_index(preedit_start as u32);
                ul_attr.set_end_index(preedit_end as u32);
                attributes.insert(ul_attr);
                layout.set_attributes(Some(&attributes));
            }

            if has_autocomplete {
                // Insert autocomplete text.
                let autocomplete_start = virtual_text.len();
                virtual_text.push_str(&text_options.autocomplete);

                // Set color for autocomplete text.
                let attributes = layout.attributes().unwrap_or_default();
                let [r, g, b] = text_options.autocomplete_color;
                let mut col_attr = AttrColor::new_foreground(r, g, b);
                col_attr.set_start_index(autocomplete_start as u32);
                col_attr.set_end_index(virtual_text.len() as u32);
                attributes.insert(col_attr);
                layout.set_attributes(Some(&attributes));
            }

            layout.set_text(&virtual_text);
        }

        // Set attributes for multi-character IME underline cursor.
        let (cursor_start, cursor_end) = text_options.cursor_pos();
        if text_options.show_cursor && cursor_end > cursor_start {
            let mut ul_attr = AttrInt::new_underline(Underline::Double);
            ul_attr.set_start_index(cursor_start as u32);
            ul_attr.set_end_index(cursor_end as u32);

            let attributes = layout.attributes().unwrap_or_default();
            attributes.insert(ul_attr);

            layout.set_attributes(Some(&attributes));
        }

        // Render text.
        self.context.move_to(position.x, text_y);
        self.context.set_source_rgb(color[0], color[1], color[2]);
        pangocairo::functions::show_layout(&self.context, layout);

        // Draw normal I-Beam cursor above text.
        if text_options.show_cursor && cursor_start == cursor_end {
            // Get cursor rect and convert it from pango coordinates.
            let (cursor_rect, _) = layout.cursor_pos(cursor_start);
            let cursor_x = position.x + cursor_rect.x() as f64 / PANGO_SCALE as f64;
            let cursor_y = text_y + cursor_rect.y() as f64 / PANGO_SCALE as f64;
            let cursor_height = cursor_rect.height() as f64 / PANGO_SCALE as f64;

            // Draw cursor line.
            self.context.move_to(cursor_x, cursor_y);
            self.context.line_to(cursor_x, cursor_y + cursor_height);
            self.context.stroke_preserve().unwrap();
        }

        // Clear selection markup attributes after rendering.
        layout.set_attributes(None);

        // Reset text to remove preedit/autocomplete.
        if let Some(text) = text_without_virtual.take() {
            layout.set_text(&text);
        }
    }

    /// Get the underlying Cairo context for direct drawing.
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Finalize the output texture.
    #[cfg_attr(feature = "profiling", profiling::function)]
    pub fn build(self) -> Texture {
        drop(self.context);

        // Transform cairo buffer from RGBA to BGRA.
        let width = self.image_surface.width() as usize;
        let height = self.image_surface.height() as usize;
        let mut data = self.image_surface.take_data().unwrap();
        for chunk in data.chunks_mut(4) {
            chunk.swap(2, 0);
        }

        Texture::new(&data, width, height)
    }
}

/// Options for text rendering.
pub struct TextOptions {
    selection: Option<Range<i32>>,
    preedit: (String, i32, i32),
    autocomplete: String,
    autocomplete_color: [u16; 3],
    text_color: [f64; 3],
    position: Position<f64>,
    size: Option<Size<i32>>,
    show_cursor: bool,
    cursor_pos: i32,
    ellipsize: bool,
}

impl TextOptions {
    pub fn new() -> Self {
        Self {
            autocomplete_color: [50_000; 3],
            text_color: [1.; 3],
            ellipsize: true,
            cursor_pos: -1,
            autocomplete: Default::default(),
            show_cursor: Default::default(),
            selection: Default::default(),
            position: Default::default(),
            preedit: Default::default(),
            size: Default::default(),
        }
    }

    /// Set text color.
    pub fn text_color(&mut self, color: [f64; 3]) {
        self.text_color = color;
    }

    /// Set text position.
    pub fn position(&mut self, position: Position<f64>) {
        self.position = position;
    }

    /// Set text size.
    pub fn size(&mut self, size: Size<i32>) {
        self.size = Some(size);
    }

    /// Show text input cursor.
    pub fn show_cursor(&mut self) {
        self.show_cursor = true;
    }

    /// Set text input cursor position.
    pub fn cursor_position(&mut self, pos: i32) {
        self.cursor_pos = pos;
    }

    /// Text selection range.
    pub fn selection(&mut self, selection: Option<Range<i32>>) {
        self.selection = selection;
    }

    /// Preedit text and cursor.
    pub fn preedit(&mut self, (text, cursor_begin, cursor_end): (String, i32, i32)) {
        self.preedit = (text, cursor_begin, cursor_end);
    }

    /// Autocomplete suggestion text.
    pub fn autocomplete(&mut self, autocomplete: String) {
        self.autocomplete = autocomplete;
    }

    /// Get cursor position.
    pub fn cursor_pos(&self) -> (i32, i32) {
        if !self.preedit.0.is_empty() {
            let cursor_pos = self.cursor_pos.max(0);
            (cursor_pos + self.preedit.1, cursor_pos + self.preedit.2)
        } else {
            (self.cursor_pos, self.cursor_pos)
        }
    }

    /// Set whether text should be truncated with an ellipsis.
    pub fn set_ellipsize(&mut self, ellipsize: bool) {
        self.ellipsize = ellipsize;
    }
}

impl Default for TextOptions {
    fn default() -> Self {
        Self::new()
    }
}

/// Font layout with font description.
pub struct TextLayout {
    layout: Layout,
    font: FontDescription,
    font_size: u8,
    scale: f64,
}

impl TextLayout {
    pub fn new(font_size: u8, scale: f64) -> Self {
        // Create pango layout.
        let image_surface = ImageSurface::create(Format::ARgb32, 0, 0).unwrap();
        let context = Context::new(&image_surface).unwrap();
        let layout = pangocairo::functions::create_layout(&context);

        // Set font description.
        let font_desc = format!("sans {}px", font_size);
        let mut font = FontDescription::from_string(&font_desc);
        font.set_absolute_size(font.size() as f64 * scale);
        layout.set_font_description(Some(&font));

        Self { layout, font, font_size, scale }
    }

    /// Update the font scale.
    pub fn set_scale(&mut self, scale: f64) {
        if scale == self.scale {
            return;
        }
        self.scale = scale;

        // Update font size.
        let pango_size = self.font_size as i32 * PANGO_SCALE;
        self.font.set_absolute_size(pango_size as f64 * scale);
        self.layout.set_font_description(Some(&self.font));
    }
}

impl Deref for TextLayout {
    type Target = Layout;

    fn deref(&self) -> &Self::Target {
        &self.layout
    }
}
