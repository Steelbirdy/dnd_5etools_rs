use std::ops::{Bound, RangeBounds};

pub(crate) fn bounds_from_range<R>(range: R) -> (Bound<usize>, Bound<usize>)
where
    R: RangeBounds<usize>,
{
    use Bound::*;

    let (lower, upper) = (range.start_bound(), range.end_bound());

    let lower = match lower {
        Unbounded => Unbounded,
        Included(n) => Included(*n),
        Excluded(n) => Included(*n + 1),
    };

    let upper = match upper {
        Unbounded => Unbounded,
        Included(n) => Included(*n),
        Excluded(n) => {
            let n = if *n == 0 { 0 } else { *n - 1 };
            Included(n)
        }
    };

    (lower, upper)
}
