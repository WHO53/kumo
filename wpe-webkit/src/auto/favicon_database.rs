// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use std::boxed::Box as Box_;

use glib::prelude::*;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "WebKitFaviconDatabase")]
    pub struct FaviconDatabase(Object<ffi::WebKitFaviconDatabase, ffi::WebKitFaviconDatabaseClass>);

    match fn {
        type_ => || ffi::webkit_favicon_database_get_type(),
    }
}

impl FaviconDatabase {
    pub const NONE: Option<&'static FaviconDatabase> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::FaviconDatabase>> Sealed for T {}
}

pub trait FaviconDatabaseExt: IsA<FaviconDatabase> + sealed::Sealed + 'static {
    #[doc(alias = "webkit_favicon_database_clear")]
    fn clear(&self) {
        unsafe {
            ffi::webkit_favicon_database_clear(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "webkit_favicon_database_get_favicon_uri")]
    #[doc(alias = "get_favicon_uri")]
    fn favicon_uri(&self, page_uri: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::webkit_favicon_database_get_favicon_uri(
                self.as_ref().to_glib_none().0,
                page_uri.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "favicon-changed")]
    fn connect_favicon_changed<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn favicon_changed_trampoline<
            P: IsA<FaviconDatabase>,
            F: Fn(&P, &str, &str) + 'static,
        >(
            this: *mut ffi::WebKitFaviconDatabase,
            page_uri: *mut libc::c_char,
            favicon_uri: *mut libc::c_char,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                FaviconDatabase::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(page_uri),
                &glib::GString::from_glib_borrow(favicon_uri),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"favicon-changed\0".as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    favicon_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<FaviconDatabase>> FaviconDatabaseExt for O {}
