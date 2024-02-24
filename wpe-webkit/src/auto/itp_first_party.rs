// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ITPFirstParty(Shared<ffi::WebKitITPFirstParty>);

    match fn {
        ref => |ptr| ffi::webkit_itp_first_party_ref(ptr),
        unref => |ptr| ffi::webkit_itp_first_party_unref(ptr),
        type_ => || ffi::webkit_itp_first_party_get_type(),
    }
}

impl ITPFirstParty {
    #[doc(alias = "webkit_itp_first_party_get_domain")]
    #[doc(alias = "get_domain")]
    pub fn domain(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_itp_first_party_get_domain(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_itp_first_party_get_last_update_time")]
    #[doc(alias = "get_last_update_time")]
    pub fn last_update_time(&self) -> Option<glib::DateTime> {
        unsafe {
            from_glib_none(ffi::webkit_itp_first_party_get_last_update_time(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_itp_first_party_get_website_data_access_allowed")]
    #[doc(alias = "get_website_data_access_allowed")]
    pub fn is_website_data_access_allowed(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_itp_first_party_get_website_data_access_allowed(
                self.to_glib_none().0,
            ))
        }
    }
}
