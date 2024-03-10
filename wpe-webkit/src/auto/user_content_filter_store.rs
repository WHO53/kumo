// This file was generated by gir (https://github.com/gtk-rs/gir)
// from /usr/share/gir-1.0
// from ../gir-files
// DO NOT EDIT

use std::boxed::Box as Box_;
use std::pin::Pin;

use glib::prelude::*;
use glib::translate::*;

use crate::UserContentFilter;

glib::wrapper! {
    #[doc(alias = "WebKitUserContentFilterStore")]
    pub struct UserContentFilterStore(Object<ffi::WebKitUserContentFilterStore, ffi::WebKitUserContentFilterStoreClass>);

    match fn {
        type_ => || ffi::webkit_user_content_filter_store_get_type(),
    }
}

impl UserContentFilterStore {
    #[doc(alias = "webkit_user_content_filter_store_new")]
    pub fn new(storage_path: &str) -> UserContentFilterStore {
        unsafe {
            from_glib_full(ffi::webkit_user_content_filter_store_new(storage_path.to_glib_none().0))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct
    /// [`UserContentFilterStore`] objects.
    ///
    /// This method returns an instance of
    /// [`UserContentFilterStoreBuilder`](crate::builders::UserContentFilterStoreBuilder)
    /// which can be used to create [`UserContentFilterStore`] objects.
    pub fn builder() -> UserContentFilterStoreBuilder {
        UserContentFilterStoreBuilder::new()
    }

    #[doc(alias = "webkit_user_content_filter_store_get_path")]
    #[doc(alias = "get_path")]
    pub fn path(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::webkit_user_content_filter_store_get_path(self.to_glib_none().0))
        }
    }

    #[doc(alias = "webkit_user_content_filter_store_load")]
    pub fn load<P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static>(
        &self,
        identifier: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context =
            (!is_main_context_owner).then(|| main_context.acquire().ok()).flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn load_trampoline<
            P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::webkit_user_content_filter_store_load_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result =
                if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = load_trampoline::<P>;
        unsafe {
            ffi::webkit_user_content_filter_store_load(
                self.to_glib_none().0,
                identifier.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn load_future(
        &self,
        identifier: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserContentFilter, glib::Error>> + 'static>>
    {
        let identifier = String::from(identifier);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.load(&identifier, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "webkit_user_content_filter_store_remove")]
    pub fn remove<P: FnOnce(Result<(), glib::Error>) + 'static>(
        &self,
        identifier: &str,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context =
            (!is_main_context_owner).then(|| main_context.acquire().ok()).flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn remove_trampoline<P: FnOnce(Result<(), glib::Error>) + 'static>(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let _ = ffi::webkit_user_content_filter_store_remove_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = remove_trampoline::<P>;
        unsafe {
            ffi::webkit_user_content_filter_store_remove(
                self.to_glib_none().0,
                identifier.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn remove_future(
        &self,
        identifier: &str,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<(), glib::Error>> + 'static>> {
        let identifier = String::from(identifier);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.remove(&identifier, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "webkit_user_content_filter_store_save")]
    pub fn save<P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static>(
        &self,
        identifier: &str,
        source: &glib::Bytes,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context =
            (!is_main_context_owner).then(|| main_context.acquire().ok()).flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn save_trampoline<
            P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::webkit_user_content_filter_store_save_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result =
                if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = save_trampoline::<P>;
        unsafe {
            ffi::webkit_user_content_filter_store_save(
                self.to_glib_none().0,
                identifier.to_glib_none().0,
                source.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn save_future(
        &self,
        identifier: &str,
        source: &glib::Bytes,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserContentFilter, glib::Error>> + 'static>>
    {
        let identifier = String::from(identifier);
        let source = source.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.save(&identifier, &source, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }

    #[doc(alias = "webkit_user_content_filter_store_save_from_file")]
    pub fn save_from_file<P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static>(
        &self,
        identifier: &str,
        file: &impl IsA<gio::File>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context =
            (!is_main_context_owner).then(|| main_context.acquire().ok()).flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn save_from_file_trampoline<
            P: FnOnce(Result<UserContentFilter, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = std::ptr::null_mut();
            let ret = ffi::webkit_user_content_filter_store_save_from_file_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result =
                if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = save_from_file_trampoline::<P>;
        unsafe {
            ffi::webkit_user_content_filter_store_save_from_file(
                self.to_glib_none().0,
                identifier.to_glib_none().0,
                file.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn save_from_file_future(
        &self,
        identifier: &str,
        file: &(impl IsA<gio::File> + Clone + 'static),
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<UserContentFilter, glib::Error>> + 'static>>
    {
        let identifier = String::from(identifier);
        let file = file.clone();
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.save_from_file(&identifier, &file, Some(cancellable), move |res| {
                send.resolve(res);
            });
        }))
    }
}

impl Default for UserContentFilterStore {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`UserContentFilterStore`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct UserContentFilterStoreBuilder {
    builder: glib::object::ObjectBuilder<'static, UserContentFilterStore>,
}

impl UserContentFilterStoreBuilder {
    fn new() -> Self {
        Self { builder: glib::object::Object::builder() }
    }

    pub fn path(self, path: impl Into<glib::GString>) -> Self {
        Self { builder: self.builder.property("path", path.into()) }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`UserContentFilterStore`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to \
                  have side effects"]
    pub fn build(self) -> UserContentFilterStore {
        self.builder.build()
    }
}
