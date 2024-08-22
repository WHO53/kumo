// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use glib::translate::*;

use crate::{ffi, BackForwardListItem};

glib::wrapper! {
    #[doc(alias = "WebKitBackForwardList")]
    pub struct BackForwardList(Object<ffi::WebKitBackForwardList, ffi::WebKitBackForwardListClass>);

    match fn {
        type_ => || ffi::webkit_back_forward_list_get_type(),
    }
}

impl BackForwardList {
    #[doc(alias = "webkit_back_forward_list_get_back_item")]
    #[doc(alias = "get_back_item")]
    pub fn back_item(&self) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_back_item(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_back_forward_list_get_back_list")]
    #[doc(alias = "get_back_list")]
    pub fn back_list(&self) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::webkit_back_forward_list_get_back_list(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_back_forward_list_get_back_list_with_limit")]
    #[doc(alias = "get_back_list_with_limit")]
    pub fn back_list_with_limit(&self, limit: u32) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::webkit_back_forward_list_get_back_list_with_limit(
                    self.to_glib_none().0,
                    limit,
                ),
            )
        }
    }

    #[doc(alias = "webkit_back_forward_list_get_current_item")]
    #[doc(alias = "get_current_item")]
    pub fn current_item(&self) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_current_item(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_back_forward_list_get_forward_item")]
    #[doc(alias = "get_forward_item")]
    pub fn forward_item(&self) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_forward_item(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_back_forward_list_get_forward_list")]
    #[doc(alias = "get_forward_list")]
    pub fn forward_list(&self) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::webkit_back_forward_list_get_forward_list(self.to_glib_none().0),
            )
        }
    }

    #[doc(alias = "webkit_back_forward_list_get_forward_list_with_limit")]
    #[doc(alias = "get_forward_list_with_limit")]
    pub fn forward_list_with_limit(&self, limit: u32) -> Vec<BackForwardListItem> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(
                ffi::webkit_back_forward_list_get_forward_list_with_limit(
                    self.to_glib_none().0,
                    limit,
                ),
            )
        }
    }

    #[doc(alias = "webkit_back_forward_list_get_length")]
    #[doc(alias = "get_length")]
    pub fn length(&self) -> u32 {
        unsafe { ffi::webkit_back_forward_list_get_length(self.to_glib_none().0) }
    }

    #[doc(alias = "webkit_back_forward_list_get_nth_item")]
    #[doc(alias = "get_nth_item")]
    pub fn nth_item(&self, index: i32) -> Option<BackForwardListItem> {
        unsafe {
            from_glib_none(ffi::webkit_back_forward_list_get_nth_item(self.to_glib_none().0, index))
        }
    }

    //#[doc(alias = "changed")]
    // pub fn connect_changed<Unsupported or ignored types>(&self, f: F) ->
    // SignalHandlerId {    Unimplemented items_removed: *.Pointer
    //}
}
