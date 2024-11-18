// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, Permission};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GSimplePermission")]
    pub struct SimplePermission(Object<ffi::GSimplePermission>) @extends Permission;

    match fn {
        type_ => || ffi::g_simple_permission_get_type(),
    }
}

impl SimplePermission {
    #[doc(alias = "g_simple_permission_new")]
    pub fn new(allowed: bool) -> SimplePermission {
        unsafe {
            Permission::from_glib_full(ffi::g_simple_permission_new(allowed.into_glib()))
                .unsafe_cast()
        }
    }
}
