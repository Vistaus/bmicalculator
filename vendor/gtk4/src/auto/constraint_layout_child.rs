// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, LayoutChild};

glib::wrapper! {
    #[doc(alias = "GtkConstraintLayoutChild")]
    pub struct ConstraintLayoutChild(Object<ffi::GtkConstraintLayoutChild, ffi::GtkConstraintLayoutChildClass>) @extends LayoutChild;

    match fn {
        type_ => || ffi::gtk_constraint_layout_child_get_type(),
    }
}

impl ConstraintLayoutChild {}
