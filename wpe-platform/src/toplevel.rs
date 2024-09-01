//! Trait for subclassing Toplevel.

use ffi::WPEToplevel;
use glib::subclass::prelude::*;
use glib::translate::IntoGlib;
use glib_sys::{gboolean, GTRUE};

use crate::Toplevel;

pub trait ToplevelImpl: ObjectImpl {
    /// Update window fullscreen state.
    fn set_fullscreen(&self, _fullscreen: bool) -> bool {
        false
    }
}

unsafe impl<T: ToplevelImpl> IsSubclassable<T> for Toplevel {
    fn class_init(class: &mut glib::Class<Self>) {
        Self::parent_class_init::<T>(class);

        let klass = class.as_mut();
        klass.set_title = None;
        klass.get_max_views = None;
        klass.get_monitor = None;
        klass.resize = None;
        klass.set_fullscreen = Some(set_fullscreen::<T>);
        klass.set_maximized = None;
        klass.get_preferred_dma_buf_formats = None;
    }
}

unsafe extern "C" fn set_fullscreen<T: ToplevelImpl>(
    toplevel: *mut WPEToplevel,
    fullscreen: gboolean,
) -> gboolean {
    let instance = &*(toplevel as *mut T::Instance);
    let fullscreened = instance.imp().set_fullscreen(fullscreen == GTRUE);
    fullscreened.into_glib()
}
