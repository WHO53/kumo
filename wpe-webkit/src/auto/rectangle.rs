// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use crate::ffi;

glib::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Rectangle(Boxed<ffi::WebKitRectangle>);

    match fn {
        copy => |ptr| glib::gobject_ffi::g_boxed_copy(ffi::webkit_rectangle_get_type(), ptr as *mut _) as *mut ffi::WebKitRectangle,
        free => |ptr| glib::gobject_ffi::g_boxed_free(ffi::webkit_rectangle_get_type(), ptr as *mut _),
        type_ => || ffi::webkit_rectangle_get_type(),
    }
}
