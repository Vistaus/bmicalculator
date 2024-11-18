// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{ffi, RenderNode};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GskClipNode")]
    pub struct ClipNode(Shared<ffi::GskClipNode>);

    match fn {
        ref => |ptr| ffi::gsk_render_node_ref(ptr as *mut ffi::GskRenderNode),
        unref => |ptr| ffi::gsk_render_node_unref(ptr as *mut ffi::GskRenderNode),
    }
}

impl StaticType for ClipNode {
    fn static_type() -> glib::Type {
        unsafe { from_glib(ffi::gsk_clip_node_get_type()) }
    }
}

impl ClipNode {
    #[doc(alias = "gsk_clip_node_new")]
    pub fn new(child: impl AsRef<RenderNode>, clip: &graphene::Rect) -> ClipNode {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gsk_clip_node_new(
                child.as_ref().to_glib_none().0,
                clip.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gsk_clip_node_get_child")]
    #[doc(alias = "get_child")]
    pub fn child(&self) -> RenderNode {
        unsafe { from_glib_none(ffi::gsk_clip_node_get_child(self.to_glib_none().0)) }
    }

    #[doc(alias = "gsk_clip_node_get_clip")]
    #[doc(alias = "get_clip")]
    pub fn clip(&self) -> graphene::Rect {
        unsafe { from_glib_none(ffi::gsk_clip_node_get_clip(self.to_glib_none().0)) }
    }
}
