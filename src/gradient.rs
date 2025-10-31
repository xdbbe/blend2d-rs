//! Linear, Radial and Conical Gradients.

use std::ptr::null;

use ffi;

use crate::{Error, ExtendMode, err_to_result};

pub struct Gradient(pub(crate) ffi::BLGradientCore);

#[repr(C)]
pub struct LinearGradientValues {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

#[repr(C)]
pub struct RadialGradientValues {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub r0: f64,
    pub r1: f64,
}

#[repr(C)]
pub struct ConicGradientValues {
    pub x0: f64,
    pub y0: f64,
    pub angle: f64,
    pub repeat: f64,
}

impl Gradient {
    #[inline]
    unsafe fn new(
        type_: ffi::BLGradientType::Type,
        values: *const ::std::os::raw::c_void,
        extend_mode: ExtendMode,
    ) -> Self {
        let mut gradient = std::mem::MaybeUninit::<ffi::BLGradientCore>::uninit();
        unsafe {
            ffi::bl_gradient_init_as(
                gradient.as_mut_ptr(),
                type_,
                values,
                extend_mode as i32,
                null(),
                0,
                null(),
            );
            Gradient(gradient.assume_init())
        }
    }
    #[inline]
    pub fn new_linear(values: &LinearGradientValues, extend_mode: ExtendMode) -> Self {
        unsafe {
            Self::new(
                ffi::BLGradientType::BL_GRADIENT_TYPE_LINEAR,
                &raw const *values as _,
                extend_mode,
            )
        }
    }
    #[inline]
    pub fn new_radial(values: &LinearGradientValues, extend_mode: ExtendMode) -> Self {
        unsafe {
            Self::new(
                ffi::BLGradientType::BL_GRADIENT_TYPE_RADIAL,
                &raw const *values as _,
                extend_mode,
            )
        }
    }
    #[inline]
    pub fn new_conic(values: &LinearGradientValues, extend_mode: ExtendMode) -> Self {
        unsafe {
            Self::new(
                ffi::BLGradientType::BL_GRADIENT_TYPE_CONIC,
                &raw const *values as _,
                extend_mode,
            )
        }
    }
    #[inline]
    pub fn add_stop_rgba32(&mut self, offset: f64, rgba32: u32) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_gradient_add_stop_rgba32(&mut self.0, offset, rgba32) })
    }
}

impl Drop for Gradient {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::bl_gradient_destroy(&mut self.0);
        }
    }
}
