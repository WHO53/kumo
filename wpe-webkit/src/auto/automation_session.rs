// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use std::boxed::Box as Box_;

use glib::prelude::*;
use glib::signal::{connect_raw, SignalHandlerId};
use glib::translate::*;

use crate::{ApplicationInfo, WebView};

glib::wrapper! {
    #[doc(alias = "WebKitAutomationSession")]
    pub struct AutomationSession(Object<ffi::WebKitAutomationSession, ffi::WebKitAutomationSessionClass>);

    match fn {
        type_ => || ffi::webkit_automation_session_get_type(),
    }
}

impl AutomationSession {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct
    /// [`AutomationSession`] objects.
    ///
    /// This method returns an instance of
    /// [`AutomationSessionBuilder`](crate::builders::AutomationSessionBuilder)
    /// which can be used to create [`AutomationSession`] objects.
    pub fn builder() -> AutomationSessionBuilder {
        AutomationSessionBuilder::new()
    }

    #[doc(alias = "webkit_automation_session_get_application_info")]
    #[doc(alias = "get_application_info")]
    pub fn application_info(&self) -> Option<ApplicationInfo> {
        unsafe {
            from_glib_none(ffi::webkit_automation_session_get_application_info(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "webkit_automation_session_get_id")]
    #[doc(alias = "get_id")]
    pub fn id(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_automation_session_get_id(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_automation_session_set_application_info")]
    pub fn set_application_info(&self, info: &ApplicationInfo) {
        unsafe {
            ffi::webkit_automation_session_set_application_info(
                self.to_glib_none().0,
                info.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "create-web-view")]
    pub fn connect_create_web_view<F: Fn(&Self) -> WebView + 'static>(
        &self,
        detail: Option<&str>,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_web_view_trampoline<
            F: Fn(&AutomationSession) -> WebView + 'static,
        >(
            this: *mut ffi::WebKitAutomationSession,
            f: glib::ffi::gpointer,
        ) -> *mut ffi::WebKitWebView {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this)) // Not checked
                .to_glib_none()
                .0
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            let detailed_signal_name = detail.map(|name| format!("create-web-view::{name}\0"));
            let signal_name: &[u8] =
                detailed_signal_name.as_ref().map_or(&b"create-web-view\0"[..], |n| n.as_bytes());
            connect_raw(
                self.as_ptr() as *mut _,
                signal_name.as_ptr() as *const _,
                Some(std::mem::transmute::<_, unsafe extern "C" fn()>(
                    create_web_view_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AutomationSession`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AutomationSessionBuilder {
    builder: glib::object::ObjectBuilder<'static, AutomationSession>,
}

impl AutomationSessionBuilder {
    fn new() -> Self {
        Self { builder: glib::object::Object::builder() }
    }

    pub fn id(self, id: impl Into<glib::GString>) -> Self {
        Self { builder: self.builder.property("id", id.into()) }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AutomationSession`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to \
                  have side effects"]
    pub fn build(self) -> AutomationSession {
        self.builder.build()
    }
}
