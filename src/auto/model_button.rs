// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use ButtonRole;
use Container;
use Widget;
use gio;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::ObjectType;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct ModelButton(Object<gtk_sys::GtkModelButton, ModelButtonClass>) @extends Button, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || gtk_sys::gtk_model_button_get_type(),
    }
}

impl ModelButton {
    pub fn new() -> ModelButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_model_button_new()).unsafe_cast()
        }
    }

    pub fn get_property_active(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"active\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_active(&self, active: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"active\0".as_ptr() as *const _, Value::from(&active).to_glib_none().0);
        }
    }

    pub fn get_property_centered(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"centered\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_centered(&self, centered: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"centered\0".as_ptr() as *const _, Value::from(&centered).to_glib_none().0);
        }
    }

    pub fn get_property_icon(&self) -> Option<gio::Icon> {
        unsafe {
            let mut value = Value::from_type(<gio::Icon as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"icon\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_icon(&self, icon: Option<&gio::Icon>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"icon\0".as_ptr() as *const _, Value::from(icon).to_glib_none().0);
        }
    }

    pub fn get_property_iconic(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"iconic\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_iconic(&self, iconic: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"iconic\0".as_ptr() as *const _, Value::from(&iconic).to_glib_none().0);
        }
    }

    pub fn get_property_inverted(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"inverted\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_inverted(&self, inverted: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"inverted\0".as_ptr() as *const _, Value::from(&inverted).to_glib_none().0);
        }
    }

    pub fn get_property_menu_name(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"menu-name\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_menu_name(&self, menu_name: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"menu-name\0".as_ptr() as *const _, Value::from(menu_name).to_glib_none().0);
        }
    }

    pub fn get_property_role(&self) -> ButtonRole {
        unsafe {
            let mut value = Value::from_type(<ButtonRole as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"role\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_role(&self, role: ButtonRole) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"role\0".as_ptr() as *const _, Value::from(&role).to_glib_none().0);
        }
    }

    pub fn get_property_text(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"text\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    pub fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"text\0".as_ptr() as *const _, Value::from(text).to_glib_none().0);
        }
    }

    pub fn get_property_use_markup(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.as_ptr() as *mut gobject_sys::GObject, b"use-markup\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    pub fn set_property_use_markup(&self, use_markup: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.as_ptr() as *mut gobject_sys::GObject, b"use-markup\0".as_ptr() as *const _, Value::from(&use_markup).to_glib_none().0);
        }
    }

    pub fn connect_property_active_notify<F: Fn(&ModelButton) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                Some(transmute(notify_active_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_centered_notify<F: Fn(&ModelButton) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::centered\0".as_ptr() as *const _,
                Some(transmute(notify_centered_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_icon_notify<F: Fn(&ModelButton) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::icon\0".as_ptr() as *const _,
                Some(transmute(notify_icon_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_iconic_notify<F: Fn(&ModelButton) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::iconic\0".as_ptr() as *const _,
                Some(transmute(notify_iconic_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_inverted_notify<F: Fn(&ModelButton) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::inverted\0".as_ptr() as *const _,
                Some(transmute(notify_inverted_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_menu_name_notify<F: Fn(&ModelButton) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::menu-name\0".as_ptr() as *const _,
                Some(transmute(notify_menu_name_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_role_notify<F: Fn(&ModelButton) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::role\0".as_ptr() as *const _,
                Some(transmute(notify_role_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_text_notify<F: Fn(&ModelButton) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text\0".as_ptr() as *const _,
                Some(transmute(notify_text_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }

    pub fn connect_property_use_markup_notify<F: Fn(&ModelButton) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-markup\0".as_ptr() as *const _,
                Some(transmute(notify_use_markup_trampoline::<F> as usize)), Box_::into_raw(f))
        }
    }
}

impl Default for ModelButton {
    fn default() -> Self {
        Self::new()
    }
}

unsafe extern "C" fn notify_active_trampoline<F: Fn(&ModelButton) + 'static>(this: *mut gtk_sys::GtkModelButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_centered_trampoline<F: Fn(&ModelButton) + 'static>(this: *mut gtk_sys::GtkModelButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_icon_trampoline<F: Fn(&ModelButton) + 'static>(this: *mut gtk_sys::GtkModelButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_iconic_trampoline<F: Fn(&ModelButton) + 'static>(this: *mut gtk_sys::GtkModelButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_inverted_trampoline<F: Fn(&ModelButton) + 'static>(this: *mut gtk_sys::GtkModelButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_menu_name_trampoline<F: Fn(&ModelButton) + 'static>(this: *mut gtk_sys::GtkModelButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_role_trampoline<F: Fn(&ModelButton) + 'static>(this: *mut gtk_sys::GtkModelButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_text_trampoline<F: Fn(&ModelButton) + 'static>(this: *mut gtk_sys::GtkModelButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

unsafe extern "C" fn notify_use_markup_trampoline<F: Fn(&ModelButton) + 'static>(this: *mut gtk_sys::GtkModelButton, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer) {
    let f: &F = &*(f as *const F);
    f(&from_glib_borrow(this))
}

impl fmt::Display for ModelButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ModelButton")
    }
}
