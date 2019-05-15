// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use Widget;
use gdk;
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
use std::boxed::Box as Box_;
use std::fmt;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct AccelLabel(Object<gtk_sys::GtkAccelLabel, gtk_sys::GtkAccelLabelClass, AccelLabelClass>) @extends Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_accel_label_get_type(),
    }
}

impl AccelLabel {
    pub fn new(string: &str) -> AccelLabel {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_accel_label_new(string.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_ACCEL_LABEL: Option<&AccelLabel> = None;

pub trait AccelLabelExt: 'static {
    fn get_accel(&self) -> (u32, gdk::ModifierType);

    fn get_accel_widget(&self) -> Option<Widget>;

    fn get_accel_width(&self) -> u32;

    fn get_label(&self) -> Option<GString>;

    fn get_use_underline(&self) -> bool;

    fn refetch(&self) -> bool;

    fn set_accel(&self, accelerator_key: u32, accelerator_mods: gdk::ModifierType);

    //fn set_accel_closure(&self, accel_closure: /*Ignored*/Option<&glib::Closure>);

    fn set_accel_widget<P: IsA<Widget>>(&self, accel_widget: Option<&P>);

    fn set_label(&self, text: &str);

    fn set_use_underline(&self, setting: bool);

    //fn get_property_accel_closure(&self) -> /*Ignored*/Option<glib::Closure>;

    fn connect_property_accel_closure_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AccelLabel>> AccelLabelExt for O {
    fn get_accel(&self) -> (u32, gdk::ModifierType) {
        unsafe {
            let mut accelerator_key = mem::uninitialized();
            let mut accelerator_mods = mem::uninitialized();
            gtk_sys::gtk_accel_label_get_accel(self.as_ref().to_glib_none().0, &mut accelerator_key, &mut accelerator_mods);
            (accelerator_key, from_glib(accelerator_mods))
        }
    }

    fn get_accel_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_accel_label_get_accel_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_accel_width(&self) -> u32 {
        unsafe {
            gtk_sys::gtk_accel_label_get_accel_width(self.as_ref().to_glib_none().0)
        }
    }

    fn get_label(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_accel_label_get_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_accel_label_get_use_underline(self.as_ref().to_glib_none().0))
        }
    }

    fn refetch(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_accel_label_refetch(self.as_ref().to_glib_none().0))
        }
    }

    fn set_accel(&self, accelerator_key: u32, accelerator_mods: gdk::ModifierType) {
        unsafe {
            gtk_sys::gtk_accel_label_set_accel(self.as_ref().to_glib_none().0, accelerator_key, accelerator_mods.to_glib());
        }
    }

    //fn set_accel_closure(&self, accel_closure: /*Ignored*/Option<&glib::Closure>) {
    //    unsafe { TODO: call gtk_sys:gtk_accel_label_set_accel_closure() }
    //}

    fn set_accel_widget<P: IsA<Widget>>(&self, accel_widget: Option<&P>) {
        unsafe {
            gtk_sys::gtk_accel_label_set_accel_widget(self.as_ref().to_glib_none().0, accel_widget.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_label(&self, text: &str) {
        unsafe {
            gtk_sys::gtk_accel_label_set_label(self.as_ref().to_glib_none().0, text.to_glib_none().0);
        }
    }

    fn set_use_underline(&self, setting: bool) {
        unsafe {
            gtk_sys::gtk_accel_label_set_use_underline(self.as_ref().to_glib_none().0, setting.to_glib());
        }
    }

    //fn get_property_accel_closure(&self) -> /*Ignored*/Option<glib::Closure> {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"accel-closure\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get()
    //    }
    //}

    fn connect_property_accel_closure_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-closure\0".as_ptr() as *const _,
                Some(transmute(notify_accel_closure_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_accel_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-widget\0".as_ptr() as *const _,
                Some(transmute(notify_accel_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute(notify_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_use_underline_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::use-underline\0".as_ptr() as *const _,
                Some(transmute(notify_use_underline_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_accel_closure_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkAccelLabel, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<AccelLabel> {
    let f: &F = &*(f as *const F);
    f(&AccelLabel::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_accel_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkAccelLabel, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<AccelLabel> {
    let f: &F = &*(f as *const F);
    f(&AccelLabel::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkAccelLabel, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<AccelLabel> {
    let f: &F = &*(f as *const F);
    f(&AccelLabel::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_use_underline_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkAccelLabel, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<AccelLabel> {
    let f: &F = &*(f as *const F);
    f(&AccelLabel::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for AccelLabel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "AccelLabel")
    }
}
