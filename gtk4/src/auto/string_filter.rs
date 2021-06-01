// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Expression;
use crate::Filter;
use crate::StringFilterMatchMode;
use glib::object::Cast;
use glib::object::ObjectType as ObjectType_;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkStringFilter")]
    pub struct StringFilter(Object<ffi::GtkStringFilter, ffi::GtkStringFilterClass>) @extends Filter;

    match fn {
        type_ => || ffi::gtk_string_filter_get_type(),
    }
}

impl StringFilter {
    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`StringFilter`].
    ///
    /// This method returns an instance of [`StringFilterBuilder`] which can be used to create a [`StringFilter`].
    pub fn builder() -> StringFilterBuilder {
        StringFilterBuilder::default()
    }

    #[doc(alias = "gtk_string_filter_get_expression")]
    #[doc(alias = "get_expression")]
    pub fn expression(&self) -> Expression {
        unsafe { from_glib_none(ffi::gtk_string_filter_get_expression(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_string_filter_get_ignore_case")]
    #[doc(alias = "get_ignore_case")]
    pub fn ignores_case(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_string_filter_get_ignore_case(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_string_filter_get_match_mode")]
    #[doc(alias = "get_match_mode")]
    pub fn match_mode(&self) -> StringFilterMatchMode {
        unsafe { from_glib(ffi::gtk_string_filter_get_match_mode(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_string_filter_get_search")]
    #[doc(alias = "get_search")]
    pub fn search(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_string_filter_get_search(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_string_filter_set_ignore_case")]
    pub fn set_ignore_case(&self, ignore_case: bool) {
        unsafe {
            ffi::gtk_string_filter_set_ignore_case(self.to_glib_none().0, ignore_case.into_glib());
        }
    }

    #[doc(alias = "gtk_string_filter_set_match_mode")]
    pub fn set_match_mode(&self, mode: StringFilterMatchMode) {
        unsafe {
            ffi::gtk_string_filter_set_match_mode(self.to_glib_none().0, mode.into_glib());
        }
    }

    #[doc(alias = "gtk_string_filter_set_search")]
    pub fn set_search(&self, search: Option<&str>) {
        unsafe {
            ffi::gtk_string_filter_set_search(self.to_glib_none().0, search.to_glib_none().0);
        }
    }

    #[doc(alias = "expression")]
    pub fn connect_expression_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_expression_trampoline<F: Fn(&StringFilter) + 'static>(
            this: *mut ffi::GtkStringFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::expression\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_expression_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ignore-case")]
    pub fn connect_ignore_case_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ignore_case_trampoline<F: Fn(&StringFilter) + 'static>(
            this: *mut ffi::GtkStringFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ignore-case\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ignore_case_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "match-mode")]
    pub fn connect_match_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_match_mode_trampoline<F: Fn(&StringFilter) + 'static>(
            this: *mut ffi::GtkStringFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::match-mode\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_match_mode_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "search")]
    pub fn connect_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_search_trampoline<F: Fn(&StringFilter) + 'static>(
            this: *mut ffi::GtkStringFilter,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::search\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_search_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`StringFilter`].
pub struct StringFilterBuilder {
    expression: Option<Expression>,
    ignore_case: Option<bool>,
    match_mode: Option<StringFilterMatchMode>,
    search: Option<String>,
}

impl StringFilterBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`StringFilterBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`StringFilter`].
    pub fn build(self) -> StringFilter {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref expression) = self.expression {
            properties.push(("expression", expression));
        }
        if let Some(ref ignore_case) = self.ignore_case {
            properties.push(("ignore-case", ignore_case));
        }
        if let Some(ref match_mode) = self.match_mode {
            properties.push(("match-mode", match_mode));
        }
        if let Some(ref search) = self.search {
            properties.push(("search", search));
        }
        glib::Object::new::<StringFilter>(&properties)
            .expect("Failed to create an instance of StringFilter")
    }

    pub fn expression(mut self, expression: &Expression) -> Self {
        self.expression = Some(expression.clone());
        self
    }

    pub fn ignore_case(mut self, ignore_case: bool) -> Self {
        self.ignore_case = Some(ignore_case);
        self
    }

    pub fn match_mode(mut self, match_mode: StringFilterMatchMode) -> Self {
        self.match_mode = Some(match_mode);
        self
    }

    pub fn search(mut self, search: &str) -> Self {
        self.search = Some(search.to_string());
        self
    }
}

impl fmt::Display for StringFilter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("StringFilter")
    }
}
