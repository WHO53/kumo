// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use glib::prelude::*;
use glib::translate::*;

use crate::Context;

glib::wrapper! {
    #[doc(alias = "JSCException")]
    pub struct Exception(Object<ffi::JSCException, ffi::JSCExceptionClass>);

    match fn {
        type_ => || ffi::jsc_exception_get_type(),
    }
}

impl Exception {
    pub const NONE: Option<&'static Exception> = None;

    #[doc(alias = "jsc_exception_new")]
    pub fn new(context: &impl IsA<Context>, message: &str) -> Exception {
        unsafe {
            from_glib_full(ffi::jsc_exception_new(
                context.as_ref().to_glib_none().0,
                message.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "jsc_exception_new_printf")]
    // pub fn new_printf(context: &impl IsA<Context>, format: &str, : /*Unknown
    // conversion*//*Unimplemented*/Basic: VarArgs) -> Exception {    unsafe {
    // TODO: call ffi:jsc_exception_new_printf() }
    //}

    //#[doc(alias = "jsc_exception_new_vprintf")]
    // pub fn new_vprintf(context: &impl IsA<Context>, format: &str, args: /*Unknown
    // conversion*//*Unimplemented*/Unsupported) -> Exception {    unsafe {
    // TODO: call ffi:jsc_exception_new_vprintf() }
    //}

    #[doc(alias = "jsc_exception_new_with_name")]
    #[doc(alias = "new_with_name")]
    pub fn with_name(context: &impl IsA<Context>, name: &str, message: &str) -> Exception {
        unsafe {
            from_glib_full(ffi::jsc_exception_new_with_name(
                context.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                message.to_glib_none().0,
            ))
        }
    }

    //#[doc(alias = "jsc_exception_new_with_name_printf")]
    //#[doc(alias = "new_with_name_printf")]
    // pub fn with_name_printf(context: &impl IsA<Context>, name: &str, format:
    // &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Exception {
    //    unsafe { TODO: call ffi:jsc_exception_new_with_name_printf() }
    //}

    //#[doc(alias = "jsc_exception_new_with_name_vprintf")]
    //#[doc(alias = "new_with_name_vprintf")]
    // pub fn with_name_vprintf(context: &impl IsA<Context>, name: &str, format:
    // &str, args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Exception
    // {    unsafe { TODO: call ffi:jsc_exception_new_with_name_vprintf() }
    //}
}

impl std::fmt::Display for Exception {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&ExceptionExt::to_str(self))
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Exception>> Sealed for T {}
}

pub trait ExceptionExt: IsA<Exception> + sealed::Sealed + 'static {
    #[doc(alias = "jsc_exception_get_backtrace_string")]
    #[doc(alias = "get_backtrace_string")]
    fn backtrace_string(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::jsc_exception_get_backtrace_string(self.as_ref().to_glib_none().0))
        }
    }

    #[doc(alias = "jsc_exception_get_column_number")]
    #[doc(alias = "get_column_number")]
    fn column_number(&self) -> u32 {
        unsafe { ffi::jsc_exception_get_column_number(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "jsc_exception_get_line_number")]
    #[doc(alias = "get_line_number")]
    fn line_number(&self) -> u32 {
        unsafe { ffi::jsc_exception_get_line_number(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "jsc_exception_get_message")]
    #[doc(alias = "get_message")]
    fn message(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::jsc_exception_get_message(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "jsc_exception_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::jsc_exception_get_name(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "jsc_exception_get_source_uri")]
    #[doc(alias = "get_source_uri")]
    fn source_uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::jsc_exception_get_source_uri(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "jsc_exception_report")]
    fn report(&self) -> Option<glib::GString> {
        unsafe { from_glib_full(ffi::jsc_exception_report(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "jsc_exception_to_string")]
    #[doc(alias = "to_string")]
    fn to_str(&self) -> glib::GString {
        unsafe { from_glib_full(ffi::jsc_exception_to_string(self.as_ref().to_glib_none().0)) }
    }
}

impl<O: IsA<Exception>> ExceptionExt for O {}
