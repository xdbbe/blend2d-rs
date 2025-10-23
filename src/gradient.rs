//! Linear, Radial and Conical Gradients.

use ffi;

use crate::{Error, err_to_result};

pub struct Gradient(pub(crate) ffi::BLGradientCore);

impl Gradient {
    pub fn new() -> Self {
        let mut gradient = std::mem::MaybeUninit::<ffi::BLGradientCore>::uninit();
        unsafe {
            ffi::bl_gradient_init(gradient.as_mut_ptr());
            Gradient(gradient.assume_init())
        }
    }
    pub fn add_stop_rgba32(&mut self, offset: f64, rgba32: u32) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_gradient_add_stop_rgba32(&mut self.0, offset, rgba32) })
    }
}
