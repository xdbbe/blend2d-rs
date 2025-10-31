//! Image loading and handling.

use std::{ffi::CStr, ptr::null};

use crate::{Error, err_to_result};
use ffi;

pub struct Image(pub(crate) ffi::BLImageCore);

use ffi::BLFormat::*;
bl_enum! {
    /// Pixel format.
    pub enum Format {
        /// 32-bit premultiplied ARGB pixel format (8-bit components).
        PRgb32 = BL_FORMAT_PRGB32,
        /// 32-bit (X)RGB pixel format (8-bit components, alpha ignored).
        XRgb32 = BL_FORMAT_XRGB32,
        /// 8-bit alpha-only pixel format.
        A8     = BL_FORMAT_A8,
    }
    Default => PRgb32
}

impl Image {
    #[inline]
    pub fn new(w: i32, h: i32, format: Format) -> Result<Self, Error> {
        let mut image = std::mem::MaybeUninit::<ffi::BLImageCore>::uninit();
        unsafe {
            err_to_result(ffi::bl_image_init_as(
                image.as_mut_ptr(),
                w,
                h,
                format as i32,
            ))?;
            Ok(Image(image.assume_init()))
        }
    }
    #[inline]
    pub fn write_to_file(&self, filename: &CStr) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_image_write_to_file(&self.0, filename.as_ptr(), null()) })
    }
    #[inline]
    pub fn read_from_file(filename: &CStr) -> Result<Self, Error> {
        let mut image = unsafe {
            let mut image = std::mem::MaybeUninit::<ffi::BLImageCore>::uninit();
            ffi::bl_image_init(image.as_mut_ptr());
            image.assume_init()
        };

        err_to_result(unsafe {
            ffi::bl_image_read_from_file(&mut image, filename.as_ptr(), null())
        })?;
        Ok(Image(image))
    }
}

impl Drop for Image {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::bl_image_destroy(&mut self.0);
        }
    }
}
