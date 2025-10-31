use std::ptr::null;

use ffi;

use crate::{err_to_result, image::Image, path::Path, Error, Gradient};

pub struct Context(ffi::BLContextCore);

use ffi::BLCompOp::*;
bl_enum! {
    pub enum CompOp {
        SrcOver = BL_COMP_OP_SRC_OVER,
        SrcCopy = BL_COMP_OP_SRC_COPY,
        SrcIn = BL_COMP_OP_SRC_IN,
        SrcOut = BL_COMP_OP_SRC_OUT,
        SrcAtop = BL_COMP_OP_SRC_ATOP,
        DstOver = BL_COMP_OP_DST_OVER,
        DstCopy = BL_COMP_OP_DST_COPY,
        DstIn = BL_COMP_OP_DST_IN,
        DstOut = BL_COMP_OP_DST_OUT,
        DstAtop = BL_COMP_OP_DST_ATOP,
        Xor = BL_COMP_OP_XOR,
        Clear = BL_COMP_OP_CLEAR,
        Plus = BL_COMP_OP_PLUS,
        Minus = BL_COMP_OP_MINUS,
        Multiply = BL_COMP_OP_MULTIPLY,
        Screen = BL_COMP_OP_SCREEN,
        Overlay = BL_COMP_OP_OVERLAY,
        Darken = BL_COMP_OP_DARKEN,
        Lighten = BL_COMP_OP_LIGHTEN,
        ColorDodge = BL_COMP_OP_COLOR_DODGE,
        ColorBurn = BL_COMP_OP_COLOR_BURN,
        LinearBurn = BL_COMP_OP_LINEAR_BURN,
        LinearLight = BL_COMP_OP_LINEAR_LIGHT,
        PinLight = BL_COMP_OP_PIN_LIGHT,
        HardLight = BL_COMP_OP_HARD_LIGHT,
        SoftLight = BL_COMP_OP_SOFT_LIGHT,
        Difference = BL_COMP_OP_DIFFERENCE,
        Exclusion = BL_COMP_OP_EXCLUSION,
    }
    Default => SrcOver
}

impl Context {
    #[inline]
    pub fn new() -> Self {
        let mut ctx = std::mem::MaybeUninit::<ffi::BLContextCore>::uninit();
        unsafe {
            ffi::bl_context_init(ctx.as_mut_ptr());
            Context(ctx.assume_init())
        }
    }
    #[inline]
    pub fn begin(&mut self, img: &mut Image) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_begin(&mut self.0, &mut img.0, null()) })
    }
    #[inline]
    pub fn end(&mut self) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_end(&mut self.0) })
    }
    #[inline]
    pub fn set_comp_op(&mut self, comp_op: CompOp) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_set_comp_op(&mut self.0, comp_op as i32) })
    }
    #[inline]
    pub fn fill_all(&mut self) -> Result<(), Error> {
        err_to_result( unsafe { ffi::bl_context_fill_all(&mut self.0) })
    }
    #[inline]
    pub fn set_fill_style_rgba32(&mut self, rgba32: u32) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_set_fill_style_rgba32(&mut self.0, rgba32) })
    }
    #[inline]
    pub fn set_fill_style_gradient(&mut self, gradient: &Gradient) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_set_fill_style(&mut self.0, &raw const gradient.0 as _) })
    }
    #[inline]
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
    #[inline]
    pub fn fill_path(&mut self, path: &Path) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_fill_geometry(
                &mut self.0,
                ffi::BLGeometryType::BL_GEOMETRY_TYPE_PATH,
                &raw const path.0 as _,
            )
        })
    }
    #[inline]
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
