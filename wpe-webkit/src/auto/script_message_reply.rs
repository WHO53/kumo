// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use glib::translate::*;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ScriptMessageReply(Shared<ffi::WebKitScriptMessageReply>);

    match fn {
        ref => |ptr| ffi::webkit_script_message_reply_ref(ptr),
        unref => |ptr| ffi::webkit_script_message_reply_unref(ptr),
        type_ => || ffi::webkit_script_message_reply_get_type(),
    }
}

impl ScriptMessageReply {
    #[doc(alias = "webkit_script_message_reply_return_error_message")]
    pub fn return_error_message(&self, error_message: &str) {
        unsafe {
            ffi::webkit_script_message_reply_return_error_message(
                self.to_glib_none().0,
                error_message.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "webkit_script_message_reply_return_value")]
    pub fn return_value(&self, reply_value: &wpe_java_script_core::Value) {
        unsafe {
            ffi::webkit_script_message_reply_return_value(
                self.to_glib_none().0,
                reply_value.to_glib_none().0,
            );
        }
    }
}
