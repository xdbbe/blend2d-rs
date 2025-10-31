use ffi;

use crate::{Error, err_to_result};

pub struct Path(pub(crate) ffi::BLPathCore);

impl Path {
    #[inline]
    pub fn move_to(&mut self, x: f64, y: f64) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_path_move_to(&mut self.0, x, y) })
    }
    #[inline]
    pub fn cubic_to(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        x3: f64,
        y3: f64,
    ) -> Result<(), Error> {
        err_to_result(unsafe { ffi::bl_path_cubic_to(&mut self.0, x1, y1, x2, y2, x3, y3) })
    }
}

impl Default for Path {
    #[inline]
    fn default() -> Self {
        let mut path = std::mem::MaybeUninit::<ffi::BLPathCore>::uninit();
        unsafe {
            ffi::bl_path_init(path.as_mut_ptr());
            Path(path.assume_init())
        }
    }
}

impl Drop for Path {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::bl_path_destroy(&mut self.0);
        }
    }
}
