// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CellRenderer;
use CellRendererAccelMode;
use CellRendererMode;
use CellRendererText;
use gdk;
use gdk_sys;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellRendererAccel(Object<gtk_sys::GtkCellRendererAccel, gtk_sys::GtkCellRendererAccelClass, CellRendererAccelClass>) @extends CellRendererText, CellRenderer;

    match fn {
        get_type => || gtk_sys::gtk_cell_renderer_accel_get_type(),
    }
}

impl CellRendererAccel {
    pub fn new() -> CellRendererAccel {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(gtk_sys::gtk_cell_renderer_accel_new()).unsafe_cast()
        }
    }
}

impl Default for CellRendererAccel {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CELL_RENDERER_ACCEL: Option<&CellRendererAccel> = None;

pub trait CellRendererAccelExt: 'static {
    fn get_property_accel_key(&self) -> u32;

    fn set_property_accel_key(&self, accel_key: u32);

    fn get_property_accel_mode(&self) -> CellRendererAccelMode;

    fn set_property_accel_mode(&self, accel_mode: CellRendererAccelMode);

    fn get_property_accel_mods(&self) -> gdk::ModifierType;

    fn set_property_accel_mods(&self, accel_mods: gdk::ModifierType);

    fn get_property_keycode(&self) -> u32;

    fn set_property_keycode(&self, keycode: u32);

    fn connect_accel_cleared<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_accel_edited<F: Fn(&Self, &str, u32, gdk::ModifierType, u32) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_mods_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_keycode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererAccel>> CellRendererAccelExt for O {
    fn get_property_accel_key(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"accel-key\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_accel_key(&self, accel_key: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"accel-key\0".as_ptr() as *const _, Value::from(&accel_key).to_glib_none().0);
        }
    }

    fn get_property_accel_mode(&self) -> CellRendererAccelMode {
        unsafe {
            let mut value = Value::from_type(<CellRendererAccelMode as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"accel-mode\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_accel_mode(&self, accel_mode: CellRendererAccelMode) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"accel-mode\0".as_ptr() as *const _, Value::from(&accel_mode).to_glib_none().0);
        }
    }

    fn get_property_accel_mods(&self) -> gdk::ModifierType {
        unsafe {
            let mut value = Value::from_type(<gdk::ModifierType as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"accel-mods\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_accel_mods(&self, accel_mods: gdk::ModifierType) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"accel-mods\0".as_ptr() as *const _, Value::from(&accel_mods).to_glib_none().0);
        }
    }

    fn get_property_keycode(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"keycode\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_keycode(&self, keycode: u32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"keycode\0".as_ptr() as *const _, Value::from(&keycode).to_glib_none().0);
        }
    }

    fn connect_accel_cleared<F: Fn(&Self, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"accel-cleared\0".as_ptr() as *const _,
                Some(transmute(accel_cleared_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_accel_edited<F: Fn(&Self, &str, u32, gdk::ModifierType, u32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"accel-edited\0".as_ptr() as *const _,
                Some(transmute(accel_edited_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_accel_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-key\0".as_ptr() as *const _,
                Some(transmute(notify_accel_key_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_accel_mode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-mode\0".as_ptr() as *const _,
                Some(transmute(notify_accel_mode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_accel_mods_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-mods\0".as_ptr() as *const _,
                Some(transmute(notify_accel_mods_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_keycode_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::keycode\0".as_ptr() as *const _,
                Some(transmute(notify_keycode_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn accel_cleared_trampoline<P, F: Fn(&P, &str) + 'static>(this: *mut gtk_sys::GtkCellRendererAccel, path_string: *mut libc::c_char, f: glib_sys::gpointer)
where P: IsA<CellRendererAccel> {
    let f: &F = &*(f as *const F);
    f(&CellRendererAccel::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(path_string))
}

unsafe extern "C" fn accel_edited_trampoline<P, F: Fn(&P, &str, u32, gdk::ModifierType, u32) + 'static>(this: *mut gtk_sys::GtkCellRendererAccel, path_string: *mut libc::c_char, accel_key: libc::c_uint, accel_mods: gdk_sys::GdkModifierType, hardware_keycode: libc::c_uint, f: glib_sys::gpointer)
where P: IsA<CellRendererAccel> {
    let f: &F = &*(f as *const F);
    f(&CellRendererAccel::from_glib_borrow(this).unsafe_cast(), &GString::from_glib_borrow(path_string), accel_key, from_glib(accel_mods), hardware_keycode)
}

unsafe extern "C" fn notify_accel_key_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkCellRendererAccel, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<CellRendererAccel> {
    let f: &F = &*(f as *const F);
    f(&CellRendererAccel::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_accel_mode_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkCellRendererAccel, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<CellRendererAccel> {
    let f: &F = &*(f as *const F);
    f(&CellRendererAccel::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_accel_mods_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkCellRendererAccel, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<CellRendererAccel> {
    let f: &F = &*(f as *const F);
    f(&CellRendererAccel::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_keycode_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkCellRendererAccel, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<CellRendererAccel> {
    let f: &F = &*(f as *const F);
    f(&CellRendererAccel::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CellRendererAccel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellRendererAccel")
    }
}
