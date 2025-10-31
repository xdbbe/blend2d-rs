use ffi;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct PointI {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct SizeI {
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Size {
    pub w: f64,
    pub h: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct BoxI {
    pub x0: i32,
    pub y0: i32,
    pub x1: i32,
    pub y1: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Box {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct RectI {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct RoundRect {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub rx: f64,
    pub ry: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Circle {
    pub cx: f64,
    pub cy: f64,
    pub r: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Ellipse {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Arc {
    pub cx: f64,
    pub cy: f64,
    pub rx: f64,
    pub ry: f64,
    pub start: f64,
    pub sweep: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Line {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Triangle {
    pub x0: f64,
    pub y0: f64,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Matrix2D([f64; 6]);

impl Matrix2D {
    #[inline]
    pub fn translation(x: f64, y: f64) -> Matrix2D {
        Matrix2D([1.0, 0.0, 0.0, 1.0, x, y])
    }
}

use ffi::BLStrokeCap::*;
bl_enum! {
    pub enum StrokeCap {
        Butt = BL_STROKE_CAP_BUTT,
        Square = BL_STROKE_CAP_SQUARE,
        Round = BL_STROKE_CAP_ROUND,
        RoundRev = BL_STROKE_CAP_ROUND_REV,
        Triangle = BL_STROKE_CAP_TRIANGLE,
        TriangleRev = BL_STROKE_CAP_TRIANGLE_REV,
    }
    Default => Butt
}
