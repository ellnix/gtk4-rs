// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files.git)
// DO NOT EDIT

use crate::Buildable;
use crate::Sorter;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkMultiSorter")]
    pub struct MultiSorter(Object<ffi::GtkMultiSorter, ffi::GtkMultiSorterClass>) @extends Sorter, @implements gio::ListModel, Buildable;

    match fn {
        type_ => || ffi::gtk_multi_sorter_get_type(),
    }
}

impl MultiSorter {
    #[doc(alias = "gtk_multi_sorter_new")]
    pub fn new() -> MultiSorter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_multi_sorter_new()) }
    }

    #[doc(alias = "gtk_multi_sorter_append")]
    pub fn append<P: IsA<Sorter>>(&self, sorter: &P) {
        unsafe {
            ffi::gtk_multi_sorter_append(self.to_glib_none().0, sorter.as_ref().to_glib_full());
        }
    }

    #[doc(alias = "gtk_multi_sorter_remove")]
    pub fn remove(&self, position: u32) {
        unsafe {
            ffi::gtk_multi_sorter_remove(self.to_glib_none().0, position);
        }
    }
}

impl Default for MultiSorter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for MultiSorter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MultiSorter")
    }
}
