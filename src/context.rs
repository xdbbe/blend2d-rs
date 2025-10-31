use std::ptr::null;

use ffi::{self, BLGeometryType, BLStrokeCapPosition, BLTransformOp};

use crate::{
    Error, Gradient, err_to_result, geometry::StrokeCap, image::Image, path::Path, pattern::Pattern,
};

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
    pub fn render(img: &mut Image, f: fn(&mut Context) -> Result<(), Error>) -> Result<(), Error> {
        let mut ctx = Self::default();
        ctx.begin(img)?;
        f(&mut ctx)?;
        ctx.end()
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
    // Fill
    #[inline]
    pub fn fill_all(&mut self) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_fill_all(&mut self.0) })
    }
    #[inline]
    pub fn set_fill_style_rgba32(&mut self, rgba32: u32) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_set_fill_style_rgba32(&mut self.0, rgba32) })
    }
    #[inline]
    pub fn set_fill_style_gradient(&mut self, gradient: &Gradient) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_set_fill_style(&mut self.0, &raw const gradient.0 as _)
        })
    }
    #[inline]
    pub fn set_fill_style_pattern(&mut self, pattern: &Pattern) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_set_fill_style(&mut self.0, &raw const pattern.0 as _)
        })
    }
    #[inline]
    pub fn fill_round_rect(
        &mut self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        rx: f64,
        ry: f64,
    ) -> Result<(), Error> {
        let rect = ffi::BLRoundRect { x, y, w, h, rx, ry };
        err_to_result(unsafe {
            ffi::bl_context_fill_geometry(
                &mut self.0,
                BLGeometryType::BL_GEOMETRY_TYPE_ROUND_RECT,
                &raw const rect as _,
            )
        })
    }
    #[inline]
    pub fn fill_path(&mut self, path: &Path) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_fill_geometry(
                &mut self.0,
                BLGeometryType::BL_GEOMETRY_TYPE_PATH,
                &raw const path.0 as _,
            )
        })
    }
    #[inline]
    pub fn fill_path_rgba32(&mut self, path: &Path, rgba32: u32) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_fill_geometry_rgba32(
                &mut self.0,
                BLGeometryType::BL_GEOMETRY_TYPE_PATH,
                &raw const path.0 as _,
                rgba32,
            )
        })
    }
    #[inline]
    pub fn fill_circle(&mut self, cx: f64, cy: f64, r: f64) -> Result<(), Error> {
        let circle = ffi::BLCircle { cx, cy, r };
        err_to_result(unsafe {
            ffi::bl_context_fill_geometry(
                &mut self.0,
                BLGeometryType::BL_GEOMETRY_TYPE_CIRCLE,
                &raw const circle as _,
            )
        })
    }
    // Stroke
    #[inline]
    pub fn set_stroke_width(&mut self, width: f64) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_set_stroke_width(&mut self.0, width) })
    }
    #[inline]
    pub fn set_stroke_style_rgba32(&mut self, rgba32: u32) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_context_set_stroke_style_rgba32(&mut self.0, rgba32) })
    }
    #[inline]
    pub fn set_stroke_style_gradient(&mut self, gradient: &Gradient) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_set_stroke_style(&mut self.0, &raw const gradient.0 as _)
        })
    }
    #[inline]
    pub fn set_stroke_start_cap(&mut self, stroke_cap: StrokeCap) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_set_stroke_cap(
                &mut self.0,
                BLStrokeCapPosition::BL_STROKE_CAP_POSITION_START,
                stroke_cap as i32,
            )
        })
    }
    #[inline]
    pub fn set_stroke_end_cap(&mut self, stroke_cap: StrokeCap) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_set_stroke_cap(
                &mut self.0,
                BLStrokeCapPosition::BL_STROKE_CAP_POSITION_END,
                stroke_cap as i32,
            )
        })
    }
    #[inline]
    pub fn stroke_path(&mut self, path: &Path) -> Result<(), Error> {
        err_to_result(unsafe {
            ffi::bl_context_stroke_geometry(
                &mut self.0,
                BLGeometryType::BL_GEOMETRY_TYPE_PATH,
                &raw const path.0 as _,
            )
        })
    }
    // Transform
    #[inline]
    pub fn rotate_around(&mut self, angle: f64, x: f64, y: f64) -> Result<(), Error> {
        err_to_result(unsafe {
            let op_data = [angle, x, y];
            ffi::bl_context_apply_transform_op(
                &mut self.0,
                BLTransformOp::BL_TRANSFORM_OP_ROTATE_PT,
                &raw const op_data as _,
            )
        })
    }
}

impl Default for Context {
    #[inline]
    fn default() -> Self {
        let mut ctx = std::mem::MaybeUninit::<ffi::BLContextCore>::uninit();
        unsafe {
            ffi::bl_context_init(ctx.as_mut_ptr());
            Context(ctx.assume_init())
        }
    }
}

impl Drop for Context {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::bl_context_destroy(&mut self.0);
        }
    }
}
