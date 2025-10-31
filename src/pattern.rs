use std::ptr;

use crate::ExtendMode;
use crate::geometry::{Matrix2D, RectI};
use crate::{Error, Image, err_to_result};

pub struct Pattern(pub(crate) ffi::BLPatternCore);

impl Pattern {
    /// Creates a new pattern that borrows the given [`Image`] immutably for its
    /// lifetime.
    pub fn new<'r, 'm, R, M>(
        image: &Image,
        area: R,
        extend_mode: ExtendMode,
        matrix: M,
    ) -> Result<Pattern, Error>
    where
        R: Into<Option<&'r RectI>>,
        M: Into<Option<&'m Matrix2D>>,
    {
        let mut pattern = std::mem::MaybeUninit::<ffi::BLPatternCore>::uninit();
        unsafe {
            err_to_result(ffi::bl_pattern_init_as(
                pattern.as_mut_ptr(),
                &image.0,
                area.into()
                    .map_or(ptr::null(), |a| a as *const _ as *const _),
                extend_mode as i32,
                matrix
                    .into()
                    .map_or(ptr::null(), |a| a as *const _ as *const _),
            ))?;
            Ok(Pattern(pattern.assume_init()))
        }
    }

    /// Sets the pattern's [`ExtendMode`].
    #[inline]
    pub fn set_extend_mode(&mut self, mode: ExtendMode) {
        unsafe { ffi::bl_pattern_set_extend_mode(&mut self.0, mode as i32) };
    }

    /// Resets the pattern's [`ExtendMode`] to the default.
    #[inline]
    pub fn reset_extend_mode(&mut self) {
        self.set_extend_mode(Default::default());
    }
}

impl TryFrom<&Image> for Pattern {
    type Error = Error;
    #[inline]
    fn try_from(image: &Image) -> Result<Self, Self::Error> {
        Self::new(image, None, Default::default(), None)
    }
}

impl PartialEq for Pattern {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::bl_pattern_equals(&self.0, &other.0) }
    }
}

impl Drop for Pattern {
    #[inline]
    fn drop(&mut self) {
        unsafe { ffi::bl_pattern_destroy(&mut self.0) };
    }
}
