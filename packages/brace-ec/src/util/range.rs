use std::cmp::{max, min};
use std::ops::{Add, Bound, Range, RangeBounds};

use num_traits::{Bounded, One};

pub fn get_range<R, T>(range: R) -> Range<T>
where
    R: RangeBounds<T>,
    T: PartialOrd + Clone + One + Bounded + Add<Output = T>,
{
    let start = match range.start_bound() {
        Bound::Included(start) => start.clone(),
        Bound::Excluded(start) => start.clone() + T::one(),
        Bound::Unbounded => T::min_value(),
    };

    let end = match range.end_bound() {
        Bound::Included(end) => end.clone() + T::one(),
        Bound::Excluded(end) => end.clone(),
        Bound::Unbounded => T::max_value(),
    };

    start..end
}

pub fn bound_range<T, B>(range: Range<T>, bounds: B) -> Range<T>
where
    B: RangeBounds<T>,
    T: Ord + Clone + One + Add<Output = T>,
{
    let start = match bounds.start_bound() {
        Bound::Included(start) => max(range.start, start.clone()),
        Bound::Excluded(start) => max(range.start, start.clone() + T::one()),
        Bound::Unbounded => range.start,
    };

    let end = match bounds.end_bound() {
        Bound::Included(end) => min(range.end, end.clone() + T::one()),
        Bound::Excluded(end) => min(range.end, end.clone()),
        Bound::Unbounded => range.end,
    };

    start..end
}

pub fn get_slice_range_mut<'a, T, R>(input: &'a mut [T], range: &R) -> &'a mut [T]
where
    R: RangeBounds<usize>,
{
    match range.start_bound() {
        Bound::Included(&start) => match range.end_bound() {
            Bound::Included(&end) => &mut input[start..=end],
            Bound::Excluded(&end) => &mut input[start..end],
            Bound::Unbounded => &mut input[start..],
        },
        Bound::Excluded(&start) => match range.end_bound() {
            Bound::Included(&end) => &mut input[(start + 1)..=end],
            Bound::Excluded(&end) => &mut input[(start + 1)..end],
            Bound::Unbounded => &mut input[(start + 1)..],
        },
        Bound::Unbounded => match range.end_bound() {
            Bound::Included(&end) => &mut input[..=end],
            Bound::Excluded(&end) => &mut input[..end],
            Bound::Unbounded => input,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{bound_range, get_range};

    #[test]
    fn test_get_range() {
        assert_eq!(get_range(..), i32::MIN..i32::MAX);
        assert_eq!(get_range(5..), 5..i32::MAX);
        assert_eq!(get_range(..10), i32::MIN..10);
        assert_eq!(get_range(..=20), i32::MIN..21);
        assert_eq!(get_range(2..8), 2..8);
        assert_eq!(get_range(4..=25), 4..26);
    }

    #[test]
    fn test_bound_range() {
        assert_eq!(bound_range(1..10, ..), 1..10);
        assert_eq!(bound_range(1..10, 0..), 1..10);
        assert_eq!(bound_range(1..10, 2..), 2..10);
        assert_eq!(bound_range(1..10, ..6), 1..6);
        assert_eq!(bound_range(1..10, ..20), 1..10);
        assert_eq!(bound_range(1..10, ..=6), 1..7);
        assert_eq!(bound_range(1..10, ..=20), 1..10);
        assert_eq!(bound_range(1..10, 0..20), 1..10);
        assert_eq!(bound_range(1..10, 2..6), 2..6);
        assert_eq!(bound_range(1..10, 0..=20), 1..10);
        assert_eq!(bound_range(1..10, 2..=6), 2..7);
    }
}
