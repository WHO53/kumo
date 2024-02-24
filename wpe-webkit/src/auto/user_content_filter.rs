// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UserContentFilter(Shared<ffi::WebKitUserContentFilter>);

    match fn {
        ref => |ptr| ffi::webkit_user_content_filter_ref(ptr),
        unref => |ptr| ffi::webkit_user_content_filter_unref(ptr),
        type_ => || ffi::webkit_user_content_filter_get_type(),
    }
}

impl UserContentFilter {
    #[doc(alias = "webkit_user_content_filter_get_identifier")]
    #[doc(alias = "get_identifier")]
    pub fn identifier(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_user_content_filter_get_identifier(self.to_glib_none().0))
        }
    }
}
