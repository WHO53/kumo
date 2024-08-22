// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use glib::prelude::*;
use glib::translate::*;

use crate::ffi;

glib::wrapper! {
    #[doc(alias = "WebKitHitTestResult")]
    pub struct HitTestResult(Object<ffi::WebKitHitTestResult, ffi::WebKitHitTestResultClass>);

    match fn {
        type_ => || ffi::webkit_hit_test_result_get_type(),
    }
}

impl HitTestResult {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct
    /// [`HitTestResult`] objects.
    ///
    /// This method returns an instance of
    /// [`HitTestResultBuilder`](crate::builders::HitTestResultBuilder) which
    /// can be used to create [`HitTestResult`] objects.
    pub fn builder() -> HitTestResultBuilder {
        HitTestResultBuilder::new()
    }

    #[doc(alias = "webkit_hit_test_result_context_is_editable")]
    pub fn context_is_editable(&self) -> bool {
        unsafe { from_glib(ffi::webkit_hit_test_result_context_is_editable(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_hit_test_result_context_is_image")]
    pub fn context_is_image(&self) -> bool {
        unsafe { from_glib(ffi::webkit_hit_test_result_context_is_image(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_hit_test_result_context_is_link")]
    pub fn context_is_link(&self) -> bool {
        unsafe { from_glib(ffi::webkit_hit_test_result_context_is_link(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_hit_test_result_context_is_media")]
    pub fn context_is_media(&self) -> bool {
        unsafe { from_glib(ffi::webkit_hit_test_result_context_is_media(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_hit_test_result_context_is_scrollbar")]
    pub fn context_is_scrollbar(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_scrollbar(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_hit_test_result_context_is_selection")]
    pub fn context_is_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_hit_test_result_context_is_selection(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_hit_test_result_get_context")]
    #[doc(alias = "get_context")]
    pub fn context(&self) -> u32 {
        unsafe { ffi::webkit_hit_test_result_get_context(self.to_glib_none().0) }
    }

    #[doc(alias = "webkit_hit_test_result_get_image_uri")]
    #[doc(alias = "get_image_uri")]
    #[doc(alias = "image-uri")]
    pub fn image_uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_hit_test_result_get_image_uri(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_hit_test_result_get_link_label")]
    #[doc(alias = "get_link_label")]
    #[doc(alias = "link-label")]
    pub fn link_label(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_hit_test_result_get_link_label(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_hit_test_result_get_link_title")]
    #[doc(alias = "get_link_title")]
    #[doc(alias = "link-title")]
    pub fn link_title(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_hit_test_result_get_link_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_hit_test_result_get_link_uri")]
    #[doc(alias = "get_link_uri")]
    #[doc(alias = "link-uri")]
    pub fn link_uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_hit_test_result_get_link_uri(self.to_glib_none().0)) }
    }

    #[doc(alias = "webkit_hit_test_result_get_media_uri")]
    #[doc(alias = "get_media_uri")]
    #[doc(alias = "media-uri")]
    pub fn media_uri(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::webkit_hit_test_result_get_media_uri(self.to_glib_none().0)) }
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`HitTestResult`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct HitTestResultBuilder {
    builder: glib::object::ObjectBuilder<'static, HitTestResult>,
}

impl HitTestResultBuilder {
    fn new() -> Self {
        Self { builder: glib::object::Object::builder() }
    }

    pub fn context(self, context: u32) -> Self {
        Self { builder: self.builder.property("context", context) }
    }

    pub fn image_uri(self, image_uri: impl Into<glib::GString>) -> Self {
        Self { builder: self.builder.property("image-uri", image_uri.into()) }
    }

    pub fn link_label(self, link_label: impl Into<glib::GString>) -> Self {
        Self { builder: self.builder.property("link-label", link_label.into()) }
    }

    pub fn link_title(self, link_title: impl Into<glib::GString>) -> Self {
        Self { builder: self.builder.property("link-title", link_title.into()) }
    }

    pub fn link_uri(self, link_uri: impl Into<glib::GString>) -> Self {
        Self { builder: self.builder.property("link-uri", link_uri.into()) }
    }

    pub fn media_uri(self, media_uri: impl Into<glib::GString>) -> Self {
        Self { builder: self.builder.property("media-uri", media_uri.into()) }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`HitTestResult`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to \
                  have side effects"]
    pub fn build(self) -> HitTestResult {
        self.builder.build()
    }
}
