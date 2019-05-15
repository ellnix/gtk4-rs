// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use AccelGroup;
use Buildable;
use Container;
use MenuShell;
use ScrollType;
use Widget;
use glib;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
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
    pub struct Menu(Object<gtk_sys::GtkMenu, gtk_sys::GtkMenuClass, MenuClass>) @extends MenuShell, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_menu_get_type(),
    }
}

impl Menu {
    pub fn new() -> Menu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_menu_new()).unsafe_cast()
        }
    }

    //pub fn new_from_model(model: /*Ignored*/&gio::MenuModel) -> Menu {
    //    unsafe { TODO: call gtk_sys:gtk_menu_new_from_model() }
    //}

    pub fn get_for_attach_widget<P: IsA<Widget>>(widget: &P) -> Vec<Widget> {
        skip_assert_initialized!();
        unsafe {
            FromGlibPtrContainer::from_glib_none(gtk_sys::gtk_menu_get_for_attach_widget(widget.as_ref().to_glib_none().0))
        }
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_MENU: Option<&Menu> = None;

pub trait MenuExt: 'static {
    //fn attach_to_widget<P: IsA<Widget>>(&self, attach_widget: &P, detacher: Option<Box<dyn FnOnce(&Widget, &Menu) + 'static>>);

    fn detach(&self);

    fn get_accel_group(&self) -> Option<AccelGroup>;

    fn get_accel_path(&self) -> Option<GString>;

    fn get_active(&self) -> Option<Widget>;

    fn get_attach_widget(&self) -> Option<Widget>;

    fn get_monitor(&self) -> i32;

    fn get_reserve_toggle_size(&self) -> bool;

    //fn place_on_monitor(&self, monitor: /*Ignored*/&gdk::Monitor);

    fn popdown(&self);

    //fn popup_at_pointer(&self, trigger_event: /*Ignored*/Option<&gdk::Event>);

    //fn popup_at_rect<P: IsA<gdk::Surface>>(&self, rect_surface: &P, rect: /*Ignored*/&gdk::Rectangle, rect_anchor: /*Ignored*/gdk::Gravity, menu_anchor: /*Ignored*/gdk::Gravity, trigger_event: /*Ignored*/Option<&gdk::Event>);

    //fn popup_at_widget<P: IsA<Widget>>(&self, widget: &P, widget_anchor: /*Ignored*/gdk::Gravity, menu_anchor: /*Ignored*/gdk::Gravity, trigger_event: /*Ignored*/Option<&gdk::Event>);

    fn reorder_child<P: IsA<Widget>>(&self, child: &P, position: i32);

    fn reposition(&self);

    fn set_accel_group<P: IsA<AccelGroup>>(&self, accel_group: Option<&P>);

    fn set_accel_path(&self, accel_path: Option<&str>);

    fn set_active(&self, index: u32);

    fn set_monitor(&self, monitor_num: i32);

    fn set_reserve_toggle_size(&self, reserve_toggle_size: bool);

    //fn get_property_anchor_hints(&self) -> /*Ignored*/gdk::AnchorHints;

    //fn set_property_anchor_hints(&self, anchor_hints: /*Ignored*/gdk::AnchorHints);

    fn set_property_attach_widget(&self, attach_widget: Option<&Widget>);

    //fn get_property_menu_type_hint(&self) -> /*Ignored*/gdk::SurfaceTypeHint;

    //fn set_property_menu_type_hint(&self, menu_type_hint: /*Ignored*/gdk::SurfaceTypeHint);

    fn get_property_rect_anchor_dx(&self) -> i32;

    fn set_property_rect_anchor_dx(&self, rect_anchor_dx: i32);

    fn get_property_rect_anchor_dy(&self) -> i32;

    fn set_property_rect_anchor_dy(&self, rect_anchor_dy: i32);

    fn connect_move_scroll<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_move_scroll(&self, scroll_type: ScrollType);

    //fn connect_popped_up<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_accel_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_anchor_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_attach_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_menu_type_hint_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rect_anchor_dx_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rect_anchor_dy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_reserve_toggle_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Menu>> MenuExt for O {
    //fn attach_to_widget<P: IsA<Widget>>(&self, attach_widget: &P, detacher: Option<Box<dyn FnOnce(&Widget, &Menu) + 'static>>) {
    //    unsafe { TODO: call gtk_sys:gtk_menu_attach_to_widget() }
    //}

    fn detach(&self) {
        unsafe {
            gtk_sys::gtk_menu_detach(self.as_ref().to_glib_none().0);
        }
    }

    fn get_accel_group(&self) -> Option<AccelGroup> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_accel_group(self.as_ref().to_glib_none().0))
        }
    }

    fn get_accel_path(&self) -> Option<GString> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_accel_path(self.as_ref().to_glib_none().0))
        }
    }

    fn get_active(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_active(self.as_ref().to_glib_none().0))
        }
    }

    fn get_attach_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(gtk_sys::gtk_menu_get_attach_widget(self.as_ref().to_glib_none().0))
        }
    }

    fn get_monitor(&self) -> i32 {
        unsafe {
            gtk_sys::gtk_menu_get_monitor(self.as_ref().to_glib_none().0)
        }
    }

    fn get_reserve_toggle_size(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_menu_get_reserve_toggle_size(self.as_ref().to_glib_none().0))
        }
    }

    //fn place_on_monitor(&self, monitor: /*Ignored*/&gdk::Monitor) {
    //    unsafe { TODO: call gtk_sys:gtk_menu_place_on_monitor() }
    //}

    fn popdown(&self) {
        unsafe {
            gtk_sys::gtk_menu_popdown(self.as_ref().to_glib_none().0);
        }
    }

    //fn popup_at_pointer(&self, trigger_event: /*Ignored*/Option<&gdk::Event>) {
    //    unsafe { TODO: call gtk_sys:gtk_menu_popup_at_pointer() }
    //}

    //fn popup_at_rect<P: IsA<gdk::Surface>>(&self, rect_surface: &P, rect: /*Ignored*/&gdk::Rectangle, rect_anchor: /*Ignored*/gdk::Gravity, menu_anchor: /*Ignored*/gdk::Gravity, trigger_event: /*Ignored*/Option<&gdk::Event>) {
    //    unsafe { TODO: call gtk_sys:gtk_menu_popup_at_rect() }
    //}

    //fn popup_at_widget<P: IsA<Widget>>(&self, widget: &P, widget_anchor: /*Ignored*/gdk::Gravity, menu_anchor: /*Ignored*/gdk::Gravity, trigger_event: /*Ignored*/Option<&gdk::Event>) {
    //    unsafe { TODO: call gtk_sys:gtk_menu_popup_at_widget() }
    //}

    fn reorder_child<P: IsA<Widget>>(&self, child: &P, position: i32) {
        unsafe {
            gtk_sys::gtk_menu_reorder_child(self.as_ref().to_glib_none().0, child.as_ref().to_glib_none().0, position);
        }
    }

    fn reposition(&self) {
        unsafe {
            gtk_sys::gtk_menu_reposition(self.as_ref().to_glib_none().0);
        }
    }

    fn set_accel_group<P: IsA<AccelGroup>>(&self, accel_group: Option<&P>) {
        unsafe {
            gtk_sys::gtk_menu_set_accel_group(self.as_ref().to_glib_none().0, accel_group.map(|p| p.as_ref()).to_glib_none().0);
        }
    }

    fn set_accel_path(&self, accel_path: Option<&str>) {
        unsafe {
            gtk_sys::gtk_menu_set_accel_path(self.as_ref().to_glib_none().0, accel_path.to_glib_none().0);
        }
    }

    fn set_active(&self, index: u32) {
        unsafe {
            gtk_sys::gtk_menu_set_active(self.as_ref().to_glib_none().0, index);
        }
    }

    fn set_monitor(&self, monitor_num: i32) {
        unsafe {
            gtk_sys::gtk_menu_set_monitor(self.as_ref().to_glib_none().0, monitor_num);
        }
    }

    fn set_reserve_toggle_size(&self, reserve_toggle_size: bool) {
        unsafe {
            gtk_sys::gtk_menu_set_reserve_toggle_size(self.as_ref().to_glib_none().0, reserve_toggle_size.to_glib());
        }
    }

    //fn get_property_anchor_hints(&self) -> /*Ignored*/gdk::AnchorHints {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"anchor-hints\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().unwrap()
    //    }
    //}

    //fn set_property_anchor_hints(&self, anchor_hints: /*Ignored*/gdk::AnchorHints) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"anchor-hints\0".as_ptr() as *const _, Value::from(&anchor_hints).to_glib_none().0);
    //    }
    //}

    fn set_property_attach_widget(&self, attach_widget: Option<&Widget>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"attach-widget\0".as_ptr() as *const _, Value::from(attach_widget).to_glib_none().0);
        }
    }

    //fn get_property_menu_type_hint(&self) -> /*Ignored*/gdk::SurfaceTypeHint {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"menu-type-hint\0".as_ptr() as *const _, value.to_glib_none_mut().0);
    //        value.get().unwrap()
    //    }
    //}

    //fn set_property_menu_type_hint(&self, menu_type_hint: /*Ignored*/gdk::SurfaceTypeHint) {
    //    unsafe {
    //        gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"menu-type-hint\0".as_ptr() as *const _, Value::from(&menu_type_hint).to_glib_none().0);
    //    }
    //}

    fn get_property_rect_anchor_dx(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dx\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_rect_anchor_dx(&self, rect_anchor_dx: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dx\0".as_ptr() as *const _, Value::from(&rect_anchor_dx).to_glib_none().0);
        }
    }

    fn get_property_rect_anchor_dy(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dy\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_rect_anchor_dy(&self, rect_anchor_dy: i32) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"rect-anchor-dy\0".as_ptr() as *const _, Value::from(&rect_anchor_dy).to_glib_none().0);
        }
    }

    fn connect_move_scroll<F: Fn(&Self, ScrollType) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"move-scroll\0".as_ptr() as *const _,
                Some(transmute(move_scroll_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn emit_move_scroll(&self, scroll_type: ScrollType) {
        let _ = unsafe { glib::Object::from_glib_borrow(self.to_glib_none().0 as *mut gobject_sys::GObject).emit("move-scroll", &[&scroll_type]).unwrap() };
    }

    //fn connect_popped_up<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Unimplemented flipped_rect: *.Pointer
    //    Unimplemented final_rect: *.Pointer
    //}

    fn connect_property_accel_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-group\0".as_ptr() as *const _,
                Some(transmute(notify_accel_group_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_accel_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::accel-path\0".as_ptr() as *const _,
                Some(transmute(notify_accel_path_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_active_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::active\0".as_ptr() as *const _,
                Some(transmute(notify_active_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_anchor_hints_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::anchor-hints\0".as_ptr() as *const _,
                Some(transmute(notify_anchor_hints_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_attach_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::attach-widget\0".as_ptr() as *const _,
                Some(transmute(notify_attach_widget_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_menu_type_hint_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::menu-type-hint\0".as_ptr() as *const _,
                Some(transmute(notify_menu_type_hint_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_monitor_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::monitor\0".as_ptr() as *const _,
                Some(transmute(notify_monitor_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rect_anchor_dx_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rect-anchor-dx\0".as_ptr() as *const _,
                Some(transmute(notify_rect_anchor_dx_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rect_anchor_dy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rect-anchor-dy\0".as_ptr() as *const _,
                Some(transmute(notify_rect_anchor_dy_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_reserve_toggle_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::reserve-toggle-size\0".as_ptr() as *const _,
                Some(transmute(notify_reserve_toggle_size_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn move_scroll_trampoline<P, F: Fn(&P, ScrollType) + 'static>(this: *mut gtk_sys::GtkMenu, scroll_type: gtk_sys::GtkScrollType, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast(), from_glib(scroll_type))
}

unsafe extern "C" fn notify_accel_group_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_accel_path_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_active_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_anchor_hints_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_attach_widget_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_menu_type_hint_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_monitor_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rect_anchor_dx_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rect_anchor_dy_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_reserve_toggle_size_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkMenu, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<Menu> {
    let f: &F = &*(f as *const F);
    f(&Menu::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Menu")
    }
}
