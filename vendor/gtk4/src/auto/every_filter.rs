// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Buildable, Filter, MultiFilter};
use glib::translate::*;

glib::wrapper! {
    #[doc(alias = "GtkEveryFilter")]
    pub struct EveryFilter(Object<ffi::GtkEveryFilter, ffi::GtkEveryFilterClass>) @extends MultiFilter, Filter, @implements gio::ListModel, Buildable;

    match fn {
        type_ => || ffi::gtk_every_filter_get_type(),
    }
}

impl EveryFilter {
    #[doc(alias = "gtk_every_filter_new")]
    pub fn new() -> EveryFilter {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_every_filter_new()) }
    }
}

impl Default for EveryFilter {
    fn default() -> Self {
        Self::new()
    }
}
