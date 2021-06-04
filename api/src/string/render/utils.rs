use super::RenderError;
use std::ops::RangeBounds;

pub fn check_arg_count<R: RangeBounds<usize>>(
    expected: R,
    found: usize,
) -> Result<(), RenderError> {
    if expected.contains(&found) {
        Ok(())
    } else {
        Err(RenderError::arg_count(expected, found))
    }
}
