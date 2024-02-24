// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use glib::translate::*;

use crate::{NavigationType, URIRequest};

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct NavigationAction(Boxed<ffi::WebKitNavigationAction>);

    match fn {
        copy => |ptr| ffi::webkit_navigation_action_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_navigation_action_free(ptr),
        type_ => || ffi::webkit_navigation_action_get_type(),
    }
}

impl NavigationAction {
    #[doc(alias = "webkit_navigation_action_get_frame_name")]
    #[doc(alias = "get_frame_name")]
    pub fn frame_name(&mut self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_navigation_action_get_frame_name(self.to_glib_none_mut().0))
        }
    }

    #[doc(alias = "webkit_navigation_action_get_modifiers")]
    #[doc(alias = "get_modifiers")]
    pub fn modifiers(&mut self) -> u32 {
        unsafe { ffi::webkit_navigation_action_get_modifiers(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "webkit_navigation_action_get_mouse_button")]
    #[doc(alias = "get_mouse_button")]
    pub fn mouse_button(&mut self) -> u32 {
        unsafe { ffi::webkit_navigation_action_get_mouse_button(self.to_glib_none_mut().0) }
    }

    #[doc(alias = "webkit_navigation_action_get_navigation_type")]
    #[doc(alias = "get_navigation_type")]
    pub fn navigation_type(&mut self) -> NavigationType {
        unsafe {
            from_glib(ffi::webkit_navigation_action_get_navigation_type(self.to_glib_none_mut().0))
        }
    }

    #[doc(alias = "webkit_navigation_action_get_request")]
    #[doc(alias = "get_request")]
    pub fn request(&mut self) -> Option<URIRequest> {
        unsafe {
            from_glib_none(ffi::webkit_navigation_action_get_request(self.to_glib_none_mut().0))
        }
    }

    #[doc(alias = "webkit_navigation_action_is_redirect")]
    pub fn is_redirect(&mut self) -> bool {
        unsafe { from_glib(ffi::webkit_navigation_action_is_redirect(self.to_glib_none_mut().0)) }
    }

    #[doc(alias = "webkit_navigation_action_is_user_gesture")]
    pub fn is_user_gesture(&mut self) -> bool {
        unsafe {
            from_glib(ffi::webkit_navigation_action_is_user_gesture(self.to_glib_none_mut().0))
        }
    }
}
