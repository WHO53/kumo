//! Trait for subclassing Display.

use std::ffi::{c_void, CStr};

use ffi::{WPEDisplay, WPEInputMethodContext, WPEView};
use glib::subclass::prelude::*;
use glib::translate::ToGlibPtr;
use glib_sys::GError;

use crate::{Display, InputMethodContext, View};

pub trait DisplayImpl: ObjectImpl {
    /// Create a new [`crate::View`].
    fn create_view(&self) -> &View;

    /// Create a new [`crate::InputMethodContext`].
    fn create_input_method_context(&self) -> &InputMethodContext;

    /// Get the underlying render node.
    fn render_node(&self) -> &'static CStr;

    /// Get a pointer to the EGL display.
    fn egl_display(&self) -> *mut c_void;
}

unsafe impl<T: DisplayImpl> IsSubclassable<T> for Display {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.connect = None;
        klass.create_view = Some(create_view::<T>);
        klass.get_egl_display = Some(get_egl_display::<T>);
        klass.get_keymap = None;
        klass.get_preferred_dma_buf_formats = None;
        klass.get_n_monitors = None;
        klass.get_monitor = None;
        klass.get_drm_device = None;
        klass.get_drm_render_node = Some(get_render_node::<T>);
        klass.create_input_method_context = Some(create_input_method_context::<T>);
    }
}

unsafe extern "C" fn create_view<T: DisplayImpl>(display: *mut WPEDisplay) -> *mut WPEView {
    let instance = &*(display as *mut T::Instance);
    instance.imp().create_view().to_glib_none().0
}

unsafe extern "C" fn get_egl_display<T: DisplayImpl>(
    display: *mut WPEDisplay,
    _error: *mut *mut GError,
) -> *mut c_void {
    let instance = &*(display as *mut T::Instance);
    instance.imp().egl_display()
}

unsafe extern "C" fn get_render_node<T: DisplayImpl>(
    display: *mut WPEDisplay,
) -> *const std::ffi::c_char {
    let instance = &*(display as *mut T::Instance);
    instance.imp().render_node().as_ptr()
}

unsafe extern "C" fn create_input_method_context<T: DisplayImpl>(
    display: *mut WPEDisplay,
) -> *mut WPEInputMethodContext {
    let instance = &*(display as *mut T::Instance);
    instance.imp().create_input_method_context().to_glib_none().0
}
