use super::Control;
use std::ffi::{CStr, CString};
use std::mem;
use libui_ffi::{self, uiControl, uiDrawTextAlign, uiLabel};

define_control! {
    /// A non-interactable piece of text.
    rust_type: Label,
    sys_type: uiLabel
}

impl Label {
    /// Create a new label with the given string as its text.
    /// Note that labels do not auto-wrap their text; they will expand as far as needed
    /// to fit.
    pub fn new(text: &str) -> Label {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            Label::from_raw(libui_ffi::uiNewLabel(c_string.as_ptr()))
        }
    }

    /// Get a copy of the existing text on the label.
    pub fn text(&self) -> String {
        unsafe {
            CStr::from_ptr(libui_ffi::uiLabelText(self.uiLabel))
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Get a reference to the existing text on the label.
    pub fn text_ref(&self) -> &CStr {
        unsafe { CStr::from_ptr(libui_ffi::uiLabelText(self.uiLabel)) }
    }

    /// Set the text on the label.
    pub fn set_text(&mut self, text: &str) {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            libui_ffi::uiLabelSetText(self.uiLabel, c_string.as_ptr())
        }
    }

    /// Set the text on the label.
    pub fn set_alignment(&mut self, alignment: TextAlignment) {
        unsafe {
            libui_ffi::uiLabelSetAlignment(self.uiLabel, alignment.into_ui_align())
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

impl TextAlignment {
    fn into_ui_align(self) -> uiDrawTextAlign {
        return match self {
            TextAlignment::Left => libui_ffi::uiDrawTextAlignLeft,
            TextAlignment::Center => libui_ffi::uiDrawTextAlignCenter,
            TextAlignment::Right => libui_ffi::uiDrawTextAlignRight,
        } as uiDrawTextAlign;
    }
}
