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

pub fn get_range_to<R>(range: R, max: usize) -> Range<usize>
where
    R: RangeBounds<usize>,
{
    match range.start_bound() {
        Bound::Included(&start) => match range.end_bound() {
            Bound::Included(&end) => start..((end + 1).min(max)),
            Bound::Excluded(&end) => start..((end).min(max)),
            Bound::Unbounded => start..max,
        },
        Bound::Excluded(&start) => match range.end_bound() {
            Bound::Included(&end) => (start + 1)..((end + 1).min(max)),
            Bound::Excluded(&end) => (start + 1)..(end.min(max)),
            Bound::Unbounded => (start + 1)..max,
        },
        Bound::Unbounded => match range.end_bound() {
            Bound::Included(&end) => 0..(end.min(max)),
            Bound::Excluded(&end) => 0..(end.min(max)),
            Bound::Unbounded => 0..max,
        },
    }
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
