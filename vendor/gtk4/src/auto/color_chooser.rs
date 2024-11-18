// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT
#![allow(deprecated)]

use crate::ffi;
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::boxed::Box as Box_;

glib::wrapper! {
    #[doc(alias = "GtkColorChooser")]
    pub struct ColorChooser(Interface<ffi::GtkColorChooser, ffi::GtkColorChooserInterface>);

    match fn {
        type_ => || ffi::gtk_color_chooser_get_type(),
    }
}

impl ColorChooser {
    pub const NONE: Option<&'static ColorChooser> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ColorChooser>> Sealed for T {}
}

pub trait ColorChooserExt: IsA<ColorChooser> + sealed::Sealed + 'static {
    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_color_chooser_get_rgba")]
    #[doc(alias = "get_rgba")]
    fn rgba(&self) -> gdk::RGBA {
        unsafe {
            let mut color = gdk::RGBA::uninitialized();
            ffi::gtk_color_chooser_get_rgba(
                self.as_ref().to_glib_none().0,
                color.to_glib_none_mut().0,
            );
            color
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_color_chooser_get_use_alpha")]
    #[doc(alias = "get_use_alpha")]
    #[doc(alias = "use-alpha")]
    fn uses_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_color_chooser_get_use_alpha(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_color_chooser_set_rgba")]
    #[doc(alias = "rgba")]
    fn set_rgba(&self, color: &gdk::RGBA) {
        unsafe {
            ffi::gtk_color_chooser_set_rgba(self.as_ref().to_glib_none().0, color.to_glib_none().0);
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[allow(deprecated)]
    #[doc(alias = "gtk_color_chooser_set_use_alpha")]
    #[doc(alias = "use-alpha")]
    fn set_use_alpha(&self, use_alpha: bool) {
        unsafe {
            ffi::gtk_color_chooser_set_use_alpha(
                self.as_ref().to_glib_none().0,
                use_alpha.into_glib(),
            );
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "color-activated")]
    fn connect_color_activated<F: Fn(&Self, &gdk::RGBA) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn color_activated_trampoline<
            P: IsA<ColorChooser>,
            F: Fn(&P, &gdk::RGBA) + 'static,
        >(
            this: *mut ffi::GtkColorChooser,
            color: *mut gdk::ffi::GdkRGBA,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                ColorChooser::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(color),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"color-activated\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    color_activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "rgba")]
    fn connect_rgba_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_rgba_trampoline<P: IsA<ColorChooser>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkColorChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ColorChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::rgba\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_rgba_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg_attr(feature = "v4_10", deprecated = "Since 4.10")]
    #[doc(alias = "use-alpha")]
    fn connect_use_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_alpha_trampoline<
            P: IsA<ColorChooser>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkColorChooser,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ColorChooser::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-alpha\0".as_ptr() as *const _,
                Some(std::mem::transmute::<*const (), unsafe extern "C" fn()>(
                    notify_use_alpha_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ColorChooser>> ColorChooserExt for O {}
