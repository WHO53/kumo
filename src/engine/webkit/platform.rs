//! WPEPlatform implementation.

use funq::StQueueHandle;
use glib::prelude::*;
use glib::subclass::types::ObjectSubclassIsExt;
use glib::Object;
use glutin::display::RawDisplay;
use smithay_client_toolkit::seat::keyboard::{Keysym, Modifiers};
use smithay_client_toolkit::seat::pointer::AxisScroll;
use wpe_platform::ffi::{WPE_BUTTON_MIDDLE, WPE_BUTTON_PRIMARY, WPE_BUTTON_SECONDARY};
use wpe_platform::{Buffer, Display, Event, EventType, InputSource, ToplevelExt, ViewExt};

use crate::engine::webkit::input_method_context::InputMethodContext;
use crate::engine::EngineId;
use crate::{Position, Size, State};

mod imp {
    use std::cell::{Cell, OnceCell};
    use std::ffi::{c_void, CStr};

    use funq::StQueueHandle;
    use glib::prelude::*;
    use glib::subclass::prelude::*;
    use glutin::display::RawDisplay;
    use smithay_client_toolkit::seat::keyboard::Modifiers;
    use wpe_platform::ffi::WPERectangle;
    use wpe_platform::{
        Buffer, DisplayImpl, InputMethodContext, Modifiers as WebKitModifiers, Toplevel,
        ToplevelImpl, View, ViewExt, ViewImpl,
    };

    use crate::engine::webkit::input_method_context::InputMethodContext as KumoInputMethodContext;
    use crate::engine::webkit::WebKitHandler;
    use crate::engine::EngineId;
    use crate::{Position, State};

    #[derive(Default)]
    pub struct WebKitDisplay {
        input_method_context: OnceCell<KumoInputMethodContext>,
        queue: OnceCell<StQueueHandle<State>>,
        view: OnceCell<super::WebKitView>,
        egl_display: OnceCell<RawDisplay>,
        engine_id: OnceCell<EngineId>,

        pointer_position: Cell<Option<Position<f64>>>,
        pointer_modifiers: Cell<Option<WebKitModifiers>>,
    }

    // Interface for functions called from Kumo.
    impl WebKitDisplay {
        /// Initialize mandatory fields; must be called once at startup.
        pub fn init(
            &self,
            queue: StQueueHandle<State>,
            engine_id: EngineId,
            egl_display: RawDisplay,
        ) {
            let _ = self.input_method_context.set(KumoInputMethodContext::new());
            let _ = self.egl_display.set(egl_display);
            let _ = self.engine_id.set(engine_id);
            let _ = self.queue.set(queue);

            // NOTE: Must be last since it uses `self.obj()`.
            let _ = self.view.set(super::WebKitView::new(&self.obj()));
        }

        /// Convert Smithay to WebKit modifiers.
        pub fn convert_modifiers(&self, modifiers: Modifiers) -> WebKitModifiers {
            let mut webkit_modifiers =
                self.pointer_modifiers.get().unwrap_or_else(|| WebKitModifiers::empty());
            if modifiers.ctrl {
                webkit_modifiers.insert(WebKitModifiers::KEYBOARD_CONTROL);
            }
            if modifiers.alt {
                webkit_modifiers.insert(WebKitModifiers::KEYBOARD_ALT);
            }
            if modifiers.shift {
                webkit_modifiers.insert(WebKitModifiers::KEYBOARD_SHIFT);
            }
            if modifiers.caps_lock {
                webkit_modifiers.insert(WebKitModifiers::KEYBOARD_CAPS_LOCK);
            }
            if modifiers.logo {
                webkit_modifiers.insert(WebKitModifiers::KEYBOARD_META);
            }
            webkit_modifiers
        }

        /// Update pointer modifiers.
        pub fn update_pointer_modifiers(&self, button: u32, down: bool) {
            let mut modifiers =
                self.pointer_modifiers.get().unwrap_or_else(|| WebKitModifiers::empty());
            match button {
                1 => modifiers.set(WebKitModifiers::POINTER_BUTTON1, down),
                2 => modifiers.set(WebKitModifiers::POINTER_BUTTON2, down),
                3 => modifiers.set(WebKitModifiers::POINTER_BUTTON3, down),
                4 => modifiers.set(WebKitModifiers::POINTER_BUTTON4, down),
                5 => modifiers.set(WebKitModifiers::POINTER_BUTTON5, down),
                _ => (),
            }
            self.pointer_modifiers.replace(Some(modifiers));
        }

        /// Update pointer position, returning the change as delta.
        pub fn set_pointer_position(&self, position: Position<f64>) -> Position<f64> {
            let delta =
                self.pointer_position.get().map_or(Position::new(0., 0.), |pos| pos - position);
            self.pointer_position.replace(Some(position));
            delta
        }

        /// Get the display's event queue.
        pub fn queue(&self) -> StQueueHandle<State> {
            self.queue.get().unwrap().clone()
        }

        /// Get the display's engine ID.
        pub fn engine_id(&self) -> EngineId {
            *self.engine_id.get().unwrap()
        }

        /// Get access to the underlying WebKit view.
        pub fn view(&self) -> &super::WebKitView {
            self.view.get().unwrap()
        }

        /// Get access to the underlying WebKit toplevel.
        pub fn toplevel(&self) -> Toplevel {
            self.view().toplevel().unwrap()
        }

        /// Get access to the underlying WebKit input method context.
        pub fn input_method_context(&self) -> &KumoInputMethodContext {
            self.input_method_context.get().unwrap()
        }
    }

    // Interface for functions called from WebKit.
    impl DisplayImpl for WebKitDisplay {
        fn create_view(&self) -> &View {
            self.view().upcast_ref()
        }

        fn create_input_method_context(&self) -> &InputMethodContext {
            self.input_method_context().upcast_ref()
        }

        fn render_node(&self) -> &'static CStr {
            // TODO: Placeholder impl.
            c"/dev/dri/renderD128"
        }

        fn egl_display(&self) -> *mut c_void {
            let RawDisplay::Egl(ptr) = self.egl_display.get().unwrap();
            ptr.cast_mut()
        }
    }

    impl ObjectImpl for WebKitDisplay {}

    #[glib::object_subclass]
    impl ObjectSubclass for WebKitDisplay {
        type ParentType = wpe_platform::Display;
        type Type = super::WebKitDisplay;

        const NAME: &'static str = "KumoWebKitDisplay";
    }

    #[derive(Default)]
    pub struct WebKitView {
        queue: OnceCell<StQueueHandle<State>>,
        engine_id: OnceCell<EngineId>,
    }

    // Interface for functions called from Kumo.
    impl WebKitView {
        /// Initialize mandatory fields; must be called once at startup.
        pub fn init(&self, display: &WebKitDisplay) {
            let _ = self.engine_id.set(display.engine_id());
            let _ = self.queue.set(display.queue());
        }
    }

    // Interface for functions called from WebKit.
    impl ViewImpl for WebKitView {
        fn render_buffer(&self, buffer: Buffer) {
            let mut queue = self.queue.get().unwrap().clone();
            let engine_id = *self.engine_id.get().unwrap();
            queue.render_buffer(engine_id, buffer);
        }

        fn set_opaque_rectangles(&self, rects: &[WPERectangle]) {
            let mut queue = self.queue.get().unwrap().clone();
            let engine_id = *self.engine_id.get().unwrap();
            queue.set_opaque_rectangles(engine_id, rects.to_vec());
        }
    }

    impl ObjectImpl for WebKitView {}

    #[glib::object_subclass]
    impl ObjectSubclass for WebKitView {
        type ParentType = wpe_platform::View;
        type Type = super::WebKitView;

        const NAME: &'static str = "KumoWebKitView";
    }

    #[derive(Default)]
    pub struct WebKitToplevel {
        queue: OnceCell<StQueueHandle<State>>,
        engine_id: OnceCell<EngineId>,
    }

    // Interface for functions called from Kumo.
    impl WebKitToplevel {
        /// Initialize mandatory fields; must be called once at startup.
        pub fn init(&self, display: &WebKitDisplay) {
            let _ = self.engine_id.set(display.engine_id());
            let _ = self.queue.set(display.queue());
        }
    }

    // Interface for functions called from WebKit.
    impl ToplevelImpl for WebKitToplevel {
        fn set_fullscreen(&self, fullscreen: bool) -> bool {
            let mut queue = self.queue.get().unwrap().clone();
            let engine_id = *self.engine_id.get().unwrap();
            queue.set_fullscreen(engine_id, fullscreen);
            fullscreen
        }
    }

    impl ObjectImpl for WebKitToplevel {}

    #[glib::object_subclass]
    impl ObjectSubclass for WebKitToplevel {
        type ParentType = wpe_platform::Toplevel;
        type Type = super::WebKitToplevel;

        const NAME: &'static str = "KumoWebKitToplevel";
    }
}

glib::wrapper! {
    pub struct WebKitDisplay(ObjectSubclass<imp::WebKitDisplay>)
        @extends wpe_platform::Display;
}

impl WebKitDisplay {
    pub fn new(
        queue: StQueueHandle<State>,
        engine_id: EngineId,
        egl_display: RawDisplay,
        size: Size,
        scale: f64,
    ) -> Self {
        let display: Self = Object::new();
        display.imp().init(queue, engine_id, egl_display);
        display.set_size(size.width as i32, size.height as i32);
        display.set_scale(scale);
        display
    }

    pub fn input_method_context(&self) -> &InputMethodContext {
        self.imp().input_method_context()
    }

    /// Update target render size.
    pub fn set_size(&self, width: i32, height: i32) {
        self.imp().view().resized(width, height);
    }

    /// Update target render scale.
    pub fn set_scale(&self, scale: f64) {
        self.imp().toplevel().scale_changed(scale);
    }

    /// Update focus state.
    pub fn set_focus(&self, focused: bool) {
        if focused {
            self.imp().view().focus_in();
        } else {
            self.imp().view().focus_out();
        }
    }

    /// Frame completion notification.
    pub fn frame_done(&self, buffer: &impl IsA<Buffer>) {
        self.imp().view().buffer_rendered(buffer);
    }

    /// Buffer release notification.
    pub fn buffer_released(&self, buffer: &impl IsA<Buffer>) {
        self.imp().view().buffer_released(buffer);
    }

    /// Send keyboard input event.
    pub fn key(&self, time: u32, raw: u32, keysym: Keysym, modifiers: Modifiers, down: bool) {
        let display_imp = self.imp();
        let view = display_imp.view();
        let event = Event::keyboard_new(
            if down { EventType::KeyboardKeyDown } else { EventType::KeyboardKeyUp },
            view,
            InputSource::Keyboard,
            time,
            display_imp.convert_modifiers(modifiers),
            raw,
            keysym.raw(),
        );
        view.event(&event);
    }

    /// Send pointer scroll event.
    pub fn pointer_axis(
        &mut self,
        time: u32,
        position: Position<f64>,
        horizontal: AxisScroll,
        vertical: AxisScroll,
        modifiers: Modifiers,
    ) {
        let display_imp = self.imp();
        let view = display_imp.view();
        let event = Event::scroll_new(
            view,
            InputSource::Mouse,
            time,
            display_imp.convert_modifiers(modifiers),
            horizontal.absolute,
            -vertical.absolute,
            horizontal.discrete == 0 && vertical.discrete == 0,
            horizontal.stop || vertical.stop,
            position.x,
            position.y,
        );
        view.event(&event);
    }

    /// Send pointer button event.
    pub fn pointer_button(
        &mut self,
        time: u32,
        position: Position<f64>,
        button: u32,
        down: bool,
        modifiers: Modifiers,
    ) {
        let button = match button {
            272 => WPE_BUTTON_PRIMARY as u32,
            273 => WPE_BUTTON_SECONDARY as u32,
            274 => WPE_BUTTON_MIDDLE as u32,
            button => button - 271,
        };

        // Update modifiers list with latest pointer button values.
        let display_imp = self.imp();
        display_imp.update_pointer_modifiers(button, down);

        let view = display_imp.view();
        let event = Event::pointer_button_new(
            if down { EventType::PointerDown } else { EventType::PointerUp },
            view,
            InputSource::Mouse,
            time,
            display_imp.convert_modifiers(modifiers),
            button,
            position.x,
            position.y,
            if down { 1 } else { 0 },
        );
        view.event(&event);
    }

    /// Send pointer motion event.
    pub fn pointer_motion(
        &mut self,
        time: u32,
        position: Position<f64>,
        modifiers: Modifiers,
        event_type: EventType,
    ) {
        let display_imp = self.imp();
        let delta = display_imp.set_pointer_position(position);

        let view = display_imp.view();
        let event = Event::pointer_move_new(
            event_type,
            view,
            InputSource::Mouse,
            time,
            display_imp.convert_modifiers(modifiers),
            position.x,
            position.y,
            delta.x,
            delta.y,
        );
        view.event(&event);
    }

    pub fn touch(
        &mut self,
        time: u32,
        id: i32,
        position: Position<f64>,
        modifiers: Modifiers,
        event_type: EventType,
    ) {
        let display_imp = self.imp();
        let view = display_imp.view();
        let event = Event::touch_new(
            event_type,
            view,
            InputSource::Touchscreen,
            time,
            display_imp.convert_modifiers(modifiers),
            id as u32,
            position.x,
            position.y,
        );
        view.event(&event);
    }
}

glib::wrapper! {
    pub struct WebKitView(ObjectSubclass<imp::WebKitView>)
        @extends wpe_platform::View;
}

impl WebKitView {
    fn new(display: &WebKitDisplay) -> Self {
        let display_obj = display.clone().upcast::<Display>();
        let view: Self = Object::builder().property("display", display_obj).build();
        view.imp().init(display.imp());

        // Initialize the view's window interface.
        let toplevel = WebKitToplevel::new(display);
        view.set_toplevel(Some(&toplevel));

        // Map the view, to let WebKit know we're ready for frames.
        view.map();

        view
    }
}

glib::wrapper! {
    pub struct WebKitToplevel(ObjectSubclass<imp::WebKitToplevel>)
        @extends wpe_platform::Toplevel;
}

impl WebKitToplevel {
    fn new(display: &WebKitDisplay) -> Self {
        let display_obj = display.clone().upcast::<Display>();
        let toplevel: Self = Object::builder().property("display", display_obj).build();
        toplevel.imp().init(display.imp());
        toplevel
    }
}
