//! Trait for subclassing View.

use std::ffi::{c_char, c_uint, CStr};
use std::slice;

use ffi::{WPEBuffer, WPERectangle, WPEView};
use glib::subclass::prelude::*;
use glib::translate::{from_glib_none, IntoGlib};
use glib::Bytes;
use glib_sys::{gboolean, GBytes, GError};

use crate::{Buffer, View};

pub trait ViewImpl: ObjectImpl {
    /// Render a new buffer.
    fn render_buffer(&self, buffer: Buffer);

    /// Update the buffer's opaque region.
    fn set_opaque_rectangles(&self, _rects: &[WPERectangle]) {}

    /// Update the cursor to a named cursor.
    fn set_cursor_from_name(&self, _name: &str) {}

    /// Update the cursor from a pixel buffer.
    fn set_cursor_from_bytes(
        &self,
        _bytes: &[u8],
        _width: u32,
        _height: u32,
        _stride: u32,
        _hotspot_x: u32,
        _hotspot_y: u32,
    ) {
    }
}

unsafe impl<T: ViewImpl> IsSubclassable<T> for View {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.render_buffer = Some(render_buffer::<T>);
        klass.set_cursor_from_name = Some(set_cursor_from_name::<T>);
        klass.set_cursor_from_bytes = Some(set_cursor_from_bytes::<T>);
        klass.set_opaque_rectangles = Some(set_opaque_rectangles::<T>);
        klass.can_be_mapped = None;
    }
}

// TODO: Try enabling damage feature flag.
unsafe extern "C" fn render_buffer<T: ViewImpl>(
    view: *mut WPEView,
    buffer: *mut WPEBuffer,
    _damage_rects: *const WPERectangle,
    _n_damage_rects: c_uint,
    _error: *mut *mut GError,
) -> gboolean {
    let buffer: Buffer = from_glib_none(buffer);
    let instance = &*(view as *mut T::Instance);
    instance.imp().render_buffer(buffer);
    true.into_glib()
}

unsafe extern "C" fn set_cursor_from_name<T: ViewImpl>(view: *mut WPEView, name: *const c_char) {
    if let Ok(name) = CStr::from_ptr(name).to_str() {
        let instance = &*(view as *mut T::Instance);
        instance.imp().set_cursor_from_name(name);
    }
}

unsafe extern "C" fn set_cursor_from_bytes<T: ViewImpl>(
    view: *mut WPEView,
    bytes: *mut GBytes,
    width: c_uint,
    height: c_uint,
    stride: c_uint,
    hotspot_x: c_uint,
    hotspot_y: c_uint,
) {
    let bytes: Bytes = from_glib_none(bytes);
    let instance = &*(view as *mut T::Instance);
    instance.imp().set_cursor_from_bytes(&bytes, width, height, stride, hotspot_x, hotspot_y);
}

unsafe extern "C" fn set_opaque_rectangles<T: ViewImpl>(
    view: *mut WPEView,
    rects: *mut WPERectangle,
    n_rects: c_uint,
) {
    let rects = slice::from_raw_parts(rects, n_rects as usize);
    let instance = &*(view as *mut T::Instance);
    instance.imp().set_opaque_rectangles(rects);
}
