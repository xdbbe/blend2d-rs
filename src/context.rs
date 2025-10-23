use std::ptr::null;

use ffi;

use crate::{err_to_result, image::Image, path::Path, BLCompOp, Error, Gradient};

pub struct Context(ffi::BLContextCore);

impl Context {
    pub fn new() -> Self {
        let mut ctx = std::mem::MaybeUninit::<ffi::BLContextCore>::uninit();
        unsafe {
            ffi::bl_context_init(ctx.as_mut_ptr());
            Context(ctx.assume_init())
        }
    }
    pub fn begin(&mut self, img: &mut Image) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_begin(&mut self.0, &mut img.0, null()) })
    }
    pub fn end(&mut self) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_end(&mut self.0) })
    }
    pub fn set_comp_op(&mut self, comp_op: BLCompOp) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_set_comp_op(&mut self.0, comp_op) })
    }
    pub fn fill_all(&mut self) -> Result<(), Error> {
        err_to_result( unsafe { ffi::bl_context_fill_all(&mut self.0) })
    }
    pub fn set_fill_style_rgba32(&mut self, rgba32: u32) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_set_fill_style_rgba32(&mut self.0, rgba32) })
    }
    pub fn set_fill_style_gradient(&mut self, gradient: &Gradient) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_set_fill_style(&mut self.0, &raw const gradient.0 as _) })
    }
    pub fn fill_round_rect(
        &mut self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rx: f64,
        ry: f64
    ) -> Result<(), Error> {
        let rect = ffi::BLRoundRect { x, y, w, h, rx, ry};
        err_to_result(unsafe { ffi::bl_context_fill_geometry(&mut self.0, ffi::BLGeometryType::BL_GEOMETRY_TYPE_ROUND_RECT, &raw const rect as _) })
    }
    pub fn fill_path(&mut self, path: &Path) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_fill_geometry(
                &mut self.0,
                ffi::BLGeometryType::BL_GEOMETRY_TYPE_PATH,
                &raw const path.0 as _,
            )
        })
    }
    pub fn fill_path_rgba32(&mut self, path: &Path, rgba32: u32) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_fill_geometry_rgba32(
                &mut self.0,
                ffi::BLGeometryType::BL_GEOMETRY_TYPE_PATH,
                &raw const path.0 as _,
                rgba32,
            )
        })
    }
}
