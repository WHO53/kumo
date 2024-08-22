// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use glib::prelude::*;
use glib::translate::*;

use crate::{ffi, BufferDMABufFormats, Display, Monitor, ToplevelState, View};

glib::wrapper! {
    #[doc(alias = "WPEToplevel")]
    pub struct Toplevel(Object<ffi::WPEToplevel, ffi::WPEToplevelClass>);

    match fn {
        type_ => || ffi::wpe_toplevel_get_type(),
    }
}

impl Toplevel {
    pub const NONE: Option<&'static Toplevel> = None;

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Toplevel`]
    /// objects.
    ///
    /// This method returns an instance of
    /// [`ToplevelBuilder`](crate::builders::ToplevelBuilder) which can be used
    /// to create [`Toplevel`] objects.
    pub fn builder() -> ToplevelBuilder {
        ToplevelBuilder::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Toplevel`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ToplevelBuilder {
    builder: glib::object::ObjectBuilder<'static, Toplevel>,
}

impl ToplevelBuilder {
    fn new() -> Self {
        Self { builder: glib::object::Object::builder() }
    }

    pub fn display(self, display: &impl IsA<Display>) -> Self {
        Self { builder: self.builder.property("display", display.clone().upcast()) }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Toplevel`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to \
                  have side effects"]
    pub fn build(self) -> Toplevel {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Toplevel>> Sealed for T {}
}

pub trait ToplevelExt: IsA<Toplevel> + sealed::Sealed + 'static {
    #[doc(alias = "wpe_toplevel_closed")]
    fn closed(&self) {
        unsafe {
            ffi::wpe_toplevel_closed(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "wpe_toplevel_foreach_view")]
    fn foreach_view<P: FnMut(&Toplevel, &View) -> bool>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&Toplevel, &View) -> bool>(
            toplevel: *mut ffi::WPEToplevel,
            view: *mut ffi::WPEView,
            user_data: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let toplevel = from_glib_borrow(toplevel);
            let view = from_glib_borrow(view);
            let callback = user_data as *mut P;
            (*callback)(&toplevel, &view).into_glib()
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::wpe_toplevel_foreach_view(
                self.as_ref().to_glib_none().0,
                func,
                super_callback0 as *const _ as *mut _,
            );
        }
    }

    #[doc(alias = "wpe_toplevel_fullscreen")]
    fn fullscreen(&self) -> bool {
        unsafe { from_glib(ffi::wpe_toplevel_fullscreen(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_toplevel_get_display")]
    #[doc(alias = "get_display")]
    fn display(&self) -> Option<Display> {
        unsafe { from_glib_none(ffi::wpe_toplevel_get_display(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_toplevel_get_max_views")]
    #[doc(alias = "get_max_views")]
    fn max_views(&self) -> u32 {
        unsafe { ffi::wpe_toplevel_get_max_views(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "wpe_toplevel_get_monitor")]
    #[doc(alias = "get_monitor")]
    fn monitor(&self) -> Option<Monitor> {
        unsafe { from_glib_none(ffi::wpe_toplevel_get_monitor(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_toplevel_get_n_views")]
    #[doc(alias = "get_n_views")]
    fn n_views(&self) -> u32 {
        unsafe { ffi::wpe_toplevel_get_n_views(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "wpe_toplevel_get_preferred_dma_buf_formats")]
    #[doc(alias = "get_preferred_dma_buf_formats")]
    fn preferred_dma_buf_formats(&self) -> Option<BufferDMABufFormats> {
        unsafe {
            from_glib_none(ffi::wpe_toplevel_get_preferred_dma_buf_formats(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "wpe_toplevel_get_scale")]
    #[doc(alias = "get_scale")]
    fn scale(&self) -> f64 {
        unsafe { ffi::wpe_toplevel_get_scale(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "wpe_toplevel_get_size")]
    #[doc(alias = "get_size")]
    fn size(&self) -> (i32, i32) {
        unsafe {
            let mut width = std::mem::MaybeUninit::uninit();
            let mut height = std::mem::MaybeUninit::uninit();
            ffi::wpe_toplevel_get_size(
                self.as_ref().to_glib_none().0,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            (width.assume_init(), height.assume_init())
        }
    }

    #[doc(alias = "wpe_toplevel_get_state")]
    #[doc(alias = "get_state")]
    fn state(&self) -> ToplevelState {
        unsafe { from_glib(ffi::wpe_toplevel_get_state(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_toplevel_maximize")]
    fn maximize(&self) -> bool {
        unsafe { from_glib(ffi::wpe_toplevel_maximize(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_toplevel_monitor_changed")]
    fn monitor_changed(&self) {
        unsafe {
            ffi::wpe_toplevel_monitor_changed(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "wpe_toplevel_preferred_dma_buf_formats_changed")]
    fn preferred_dma_buf_formats_changed(&self) {
        unsafe {
            ffi::wpe_toplevel_preferred_dma_buf_formats_changed(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "wpe_toplevel_resize")]
    fn resize(&self, width: i32, height: i32) -> bool {
        unsafe {
            from_glib(ffi::wpe_toplevel_resize(self.as_ref().to_glib_none().0, width, height))
        }
    }

    #[doc(alias = "wpe_toplevel_resized")]
    fn resized(&self, width: i32, height: i32) {
        unsafe {
            ffi::wpe_toplevel_resized(self.as_ref().to_glib_none().0, width, height);
        }
    }

    #[doc(alias = "wpe_toplevel_scale_changed")]
    fn scale_changed(&self, scale: f64) {
        unsafe {
            ffi::wpe_toplevel_scale_changed(self.as_ref().to_glib_none().0, scale);
        }
    }

    #[doc(alias = "wpe_toplevel_set_title")]
    fn set_title(&self, title: Option<&str>) {
        unsafe {
            ffi::wpe_toplevel_set_title(self.as_ref().to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[doc(alias = "wpe_toplevel_state_changed")]
    fn state_changed(&self, state: ToplevelState) {
        unsafe {
            ffi::wpe_toplevel_state_changed(self.as_ref().to_glib_none().0, state.into_glib());
        }
    }

    #[doc(alias = "wpe_toplevel_unfullscreen")]
    fn unfullscreen(&self) -> bool {
        unsafe { from_glib(ffi::wpe_toplevel_unfullscreen(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "wpe_toplevel_unmaximize")]
    fn unmaximize(&self) -> bool {
        unsafe { from_glib(ffi::wpe_toplevel_unmaximize(self.as_ref().to_glib_none().0)) }
    }
}

impl<O: IsA<Toplevel>> ToplevelExt for O {}
