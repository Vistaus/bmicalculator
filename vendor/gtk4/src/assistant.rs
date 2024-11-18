// Take a look at the license at the top of the repository in the LICENSE file.

use glib::translate::*;

use crate::Assistant;

impl Assistant {
    #[doc(alias = "gtk_assistant_set_forward_page_func")]
    #[doc(alias = "set_forward_page_func")]
    pub fn unset_forward_page_func(&self) {
        unsafe {
            crate::ffi::gtk_assistant_set_forward_page_func(
                self.to_glib_none().0,
                None,
                std::ptr::null_mut(),
                None,
            );
        }
    }
}
