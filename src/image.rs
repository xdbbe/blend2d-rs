//! Image loading and handling.

use std::{ffi::CStr, ptr::null};

use crate::{BLFormat, Error, err_to_result};
use ffi;

pub struct Image(pub(crate) ffi::BLImageCore);

impl Image {
    pub fn new(w: i32, h: i32, format: BLFormat) -> Result<Self, Error> {
        let mut image = std::mem::MaybeUninit::<ffi::BLImageCore>::uninit();
        unsafe {
            err_to_result(ffi::bl_image_init_as(image.as_mut_ptr(), w, h, format))?;
            Ok(Image(image.assume_init()))
        }
    }
    pub fn write_to_file(&mut self, filename: &CStr) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_image_write_to_file(&mut self.0, filename.as_ptr(), null())
        })
    }
}