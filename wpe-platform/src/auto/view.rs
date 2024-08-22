// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use std::boxed::Box as Box_;

use glib::prelude::*;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;

use crate::{
    ffi, Buffer, BufferDMABufFormats, Display, Event, GestureController, Monitor, Toplevel,
    ToplevelState,
};

glib::wrapper! {
    #[doc(alias = "WPEView")]
    pub struct View(Object<ffi::WPEView, ffi::WPEViewClass>);

    match fn {
        type_ => || ffi::wpe_view_get_type(),
    }
}

impl View {
    pub const NONE: Option<&'static View> = None;

    #[doc(alias = "wpe_view_new")]
    pub fn new(display: &impl IsA<Display>) -> View {
        unsafe { from_glib_full(ffi::wpe_view_new(display.as_ref().to_glib_none().0)) }
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::View>> Sealed for T {}
}

pub trait ViewExt: IsA<View> + sealed::Sealed + 'static {
    #[doc(alias = "wpe_view_buffer_released")]
    fn buffer_released(&self, buffer: &impl IsA<Buffer>) {
        unsafe {
            ffi::wpe_view_buffer_released(
                self.as_ref().to_glib_none().0,
                buffer.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "wpe_view_buffer_rendered")]
    fn buffer_rendered(&self, buffer: &impl IsA<Buffer>) {
        unsafe {
            ffi::wpe_view_buffer_rendered(
                self.as_ref().to_glib_none().0,
                buffer.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "wpe_view_closed")]
    fn closed(&self) {
        unsafe {
            ffi::wpe_view_closed(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "wpe_view_compute_press_count")]
    fn compute_press_count(&self, x: f64, y: f64, button: u32, time: u32) -> u32 {
        unsafe {
            ffi::wpe_view_compute_press_count(self.as_ref().to_glib_none().0, x, y, button, time)
        }
    }

    #[doc(alias = "wpe_view_event")]
    fn event(&self, event: &Event) {
        unsafe {
            ffi::wpe_view_event(self.as_ref().to_glib_none().0, event.to_glib_none().0);
        }
    }

    #[doc(alias = "wpe_view_focus_in")]
    fn focus_in(&self) {
        unsafe {
            ffi::wpe_view_focus_in(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "wpe_view_focus_out")]
    fn focus_out(&self) {
        unsafe {
            ffi::wpe_view_focus_out(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "wpe_view_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::wpe_view_get_display(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_view_get_gesture_controller")]
    #[doc(alias = "get_gesture_controller")]
    fn gesture_controller(&self) -> Option<GestureController> {
        unsafe {
            from_glib_none(ffi::wpe_view_get_gesture_controller(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "wpe_view_get_has_focus")]
    #[doc(alias = "get_has_focus")]
    #[doc(alias = "has-focus")]
    fn has_focus(&self) -> bool {
        unsafe { from_glib(ffi::wpe_view_get_has_focus(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_view_get_height")]
    #[doc(alias = "get_height")]
    fn height(&self) -> i32 {
        unsafe { ffi::wpe_view_get_height(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "wpe_view_get_mapped")]
    #[doc(alias = "get_mapped")]
    #[doc(alias = "mapped")]
    fn is_mapped(&self) -> bool {
        unsafe { from_glib(ffi::wpe_view_get_mapped(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_view_get_monitor")]
    #[doc(alias = "get_monitor")]
    fn monitor(&self) -> Option<Monitor> {
        unsafe { from_glib_none(ffi::wpe_view_get_monitor(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_view_get_preferred_dma_buf_formats")]
    #[doc(alias = "get_preferred_dma_buf_formats")]
    fn preferred_dma_buf_formats(&self) -> Option<BufferDMABufFormats> {
        unsafe {
            from_glib_none(ffi::wpe_view_get_preferred_dma_buf_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "wpe_view_get_scale")]
    #[doc(alias = "get_scale")]
    fn scale(&self) -> f64 {
        unsafe { ffi::wpe_view_get_scale(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "wpe_view_get_toplevel")]
    #[doc(alias = "get_toplevel")]
    fn toplevel(&self) -> Option<Toplevel> {
        unsafe { from_glib_none(ffi::wpe_view_get_toplevel(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_view_get_toplevel_state")]
    #[doc(alias = "get_toplevel_state")]
    #[doc(alias = "toplevel-state")]
    fn toplevel_state(&self) -> ToplevelState {
        unsafe { from_glib(ffi::wpe_view_get_toplevel_state(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_view_get_visible")]
    #[doc(alias = "get_visible")]
    #[doc(alias = "visible")]
    fn is_visible(&self) -> bool {
        unsafe { from_glib(ffi::wpe_view_get_visible(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_view_get_width")]
    #[doc(alias = "get_width")]
    fn width(&self) -> i32 {
        unsafe { ffi::wpe_view_get_width(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "wpe_view_map")]
    fn map(&self) {
        unsafe {
            ffi::wpe_view_map(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "wpe_view_resized")]
    fn resized(&self, width: i32, height: i32) {
        unsafe {
            ffi::wpe_view_resized(self.as_ref().to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "wpe_view_set_cursor_from_bytes")]
    fn set_cursor_from_bytes(
        &self,
        bytes: &glib::Bytes,
        width: u32,
        height: u32,
        stride: u32,
        hotspot_x: u32,
        hotspot_y: u32,
    ) {
        unsafe {
            ffi::wpe_view_set_cursor_from_bytes(
                self.as_ref().to_glib_none().0,
                bytes.to_glib_none().0,
                width,
                height,
                stride,
                hotspot_x,
                hotspot_y,
            );
        }
    }

    #[doc(alias = "wpe_view_set_cursor_from_name")]
    fn set_cursor_from_name(&self, name: &str) {
        unsafe {
            ffi::wpe_view_set_cursor_from_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "wpe_view_set_gesture_controller")]
    fn set_gesture_controller(&self, controller: Option<&impl IsA<GestureController>>) {
        unsafe {
            ffi::wpe_view_set_gesture_controller(
                self.as_ref().to_glib_none().0,
                controller.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "wpe_view_set_toplevel")]
    #[doc(alias = "toplevel")]
    fn set_toplevel(&self, toplevel: Option<&impl IsA<Toplevel>>) {
        unsafe {
            ffi::wpe_view_set_toplevel(
                self.as_ref().to_glib_none().0,
                toplevel.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "wpe_view_set_visible")]
    #[doc(alias = "visible")]
    fn set_visible(&self, visible: bool) {
        unsafe {
            ffi::wpe_view_set_visible(self.as_ref().to_glib_none().0, visible.into_glib());
        }
    }

    #[doc(alias = "wpe_view_unmap")]
    fn unmap(&self) {
        unsafe {
            ffi::wpe_view_unmap(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "buffer-released")]
    fn connect_buffer_released<F: Fn(&Self, &Buffer) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn buffer_released_trampoline<
            P: IsA<View>,
            F: Fn(&P, &Buffer) + 'static,
        >(
            this: *mut ffi::WPEView,
            buffer: *mut ffi::WPEBuffer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(buffer))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"buffer-released\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    buffer_released_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "buffer-rendered")]
    fn connect_buffer_rendered<F: Fn(&Self, &Buffer) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn buffer_rendered_trampoline<
            P: IsA<View>,
            F: Fn(&P, &Buffer) + 'static,
        >(
            this: *mut ffi::WPEView,
            buffer: *mut ffi::WPEBuffer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(buffer))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"buffer-rendered\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    buffer_rendered_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "closed")]
    fn connect_closed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn closed_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"closed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    closed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "event")]
    fn connect_event<F: Fn(&Self, &Event) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn event_trampoline<P: IsA<View>, F: Fn(&P, &Event) -> bool + 'static>(
            this: *mut ffi::WPEView,
            event: *mut ffi::WPEEvent,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(event)).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"event\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    event_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "preferred-dma-buf-formats-changed")]
    fn connect_preferred_dma_buf_formats_changed<F: Fn(&Self) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn preferred_dma_buf_formats_changed_trampoline<
            P: IsA<View>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::WPEView,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"preferred-dma-buf-formats-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    preferred_dma_buf_formats_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resized")]
    fn connect_resized<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn resized_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"resized\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    resized_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "toplevel-state-changed")]
    fn connect_toplevel_state_changed<F: Fn(&Self, ToplevelState) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn toplevel_state_changed_trampoline<
            P: IsA<View>,
            F: Fn(&P, ToplevelState) + 'static,
        >(
            this: *mut ffi::WPEView,
            previous_state: ffi::WPEToplevelState,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref(), from_glib(previous_state))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toplevel-state-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    toplevel_state_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-focus")]
    fn connect_has_focus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_focus_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-focus\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_has_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "height")]
    fn connect_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_height_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::height\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_height_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "mapped")]
    fn connect_mapped_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_mapped_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::mapped\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_mapped_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "monitor")]
    fn connect_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_monitor_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::monitor\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_monitor_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "scale")]
    fn connect_scale_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_scale_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::scale\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_scale_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "toplevel")]
    fn connect_toplevel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_toplevel_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::toplevel\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_toplevel_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "toplevel-state")]
    fn connect_toplevel_state_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_toplevel_state_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::toplevel-state\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_toplevel_state_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "visible")]
    fn connect_visible_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_visible_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::visible\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_visible_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "width")]
    fn connect_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_width_trampoline<P: IsA<View>, F: Fn(&P) + 'static>(
            this: *mut ffi::WPEView,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(View::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::width\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_width_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<View>> ViewExt for O {}
