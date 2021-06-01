// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use glib::translate::*;
use std::fmt;
use x11::xlib;

glib::wrapper! {
    #[doc(alias = "GdkX11Monitor")]
    pub struct X11Monitor(Object<ffi::GdkX11Monitor, ffi::GdkX11MonitorClass>) @extends gdk::Monitor;

    match fn {
        type_ => || ffi::gdk_x11_monitor_get_type(),
    }
}

impl X11Monitor {
    #[doc(alias = "gdk_x11_monitor_get_output")]
    #[doc(alias = "get_output")]
    pub fn output(&self) -> xlib::XID {
        unsafe { ffi::gdk_x11_monitor_get_output(self.to_glib_none().0) }
    }

    #[doc(alias = "gdk_x11_monitor_get_workarea")]
    #[doc(alias = "get_workarea")]
    pub fn workarea(&self) -> gdk::Rectangle {
        unsafe {
            let mut workarea = gdk::Rectangle::uninitialized();
            ffi::gdk_x11_monitor_get_workarea(self.to_glib_none().0, workarea.to_glib_none_mut().0);
            workarea
        }
    }
}

impl fmt::Display for X11Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("X11Monitor")
    }
}
