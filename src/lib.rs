#[macro_use]
mod macros;

mod error;
pub use error::Error;
pub(crate) use error::err_to_result;

pub mod context;
pub mod gradient;
pub mod image;
pub mod path;

pub use context::Context;
pub use gradient::Gradient;
pub use image::Image;
pub use path::Path;

use ffi::BLExtendMode::*;
bl_enum! {
    pub enum ExtendMode {
        PadXPadY =         BL_EXTEND_MODE_PAD_X_PAD_Y,
        RepeatXRepeatY =   BL_EXTEND_MODE_REPEAT_X_REPEAT_Y,
        ReflectXReflectY = BL_EXTEND_MODE_REFLECT_X_REFLECT_Y,
        PadXRepeatY =      BL_EXTEND_MODE_PAD_X_REPEAT_Y,
        PadXReflectY =     BL_EXTEND_MODE_PAD_X_REFLECT_Y,
        RepeatXPadY =      BL_EXTEND_MODE_REPEAT_X_PAD_Y,
        RepeatXReflectY =  BL_EXTEND_MODE_REPEAT_X_REFLECT_Y,
        ReflectXPadY =     BL_EXTEND_MODE_REFLECT_X_PAD_Y,
        ReflectXRepeatY =  BL_EXTEND_MODE_REFLECT_X_REPEAT_Y,
    }
    Default => PadXPadY
}
