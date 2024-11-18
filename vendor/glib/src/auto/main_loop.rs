// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, translate::*, MainContext};

crate::wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MainLoop(Shared<ffi::GMainLoop>);

    match fn {
        ref => |ptr| ffi::g_main_loop_ref(ptr),
        unref => |ptr| ffi::g_main_loop_unref(ptr),
        type_ => || ffi::g_main_loop_get_type(),
    }
}

impl MainLoop {
    #[doc(alias = "g_main_loop_new")]
    pub fn new(context: Option<&MainContext>, is_running: bool) -> MainLoop {
        unsafe {
            from_glib_full(ffi::g_main_loop_new(
                context.to_glib_none().0,
                is_running.into_glib(),
            ))
        }
    }

    #[doc(alias = "g_main_loop_get_context")]
    #[doc(alias = "get_context")]
    pub fn context(&self) -> MainContext {
        unsafe { from_glib_none(ffi::g_main_loop_get_context(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_main_loop_is_running")]
    pub fn is_running(&self) -> bool {
        unsafe { from_glib(ffi::g_main_loop_is_running(self.to_glib_none().0)) }
    }

    #[doc(alias = "g_main_loop_quit")]
    pub fn quit(&self) {
        unsafe {
            ffi::g_main_loop_quit(self.to_glib_none().0);
        }
    }

    #[doc(alias = "g_main_loop_run")]
    pub fn run(&self) {
        unsafe {
            ffi::g_main_loop_run(self.to_glib_none().0);
        }
    }
}

unsafe impl Send for MainLoop {}
unsafe impl Sync for MainLoop {}
