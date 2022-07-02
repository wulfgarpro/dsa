//! # sort algorithms.
//!
//! `sort` defines various sorting algorithms on generic types.

/// Repeatedly step through the list and compare adjacent elements and swap them
/// if they are in the wrong order. Repeat the pass through until the list is
/// sorted.
///
/// Worst-case performance: O(n^2) comparisons, O(n^2) swaps.
/// Best-case performance: O(n) comparisons, O(1) swaps.
/// Worst-case space complexity: O(n) total, O(1) auxiliary.
pub fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
    loop {
        let mut swapped = false;
        for i in 0..list.len() - 1 {
            if list[i] > list[i + 1] {
                list.swap(i, i + 1);
                swapped = true;
            }
        }
        // If no swaps occurred, the list is sorted and no further pass through is
        // necessary.
        if !swapped {
            break;
        }
    }
}

/// TODO: Document algorithm.
#[allow(dead_code)]
pub fn insertion_sort<T: Copy + PartialOrd>(_list: &mut [T]) {
    todo!()
}

/// TODO: Document algorithm.
/// TODO: Non semi-in-place.
pub fn merge_sort<T: Copy + PartialOrd>(items: &mut [T]) {
    let len = items.len();

    // Base case.
    if len < 2 {
        return;
    }

    let mut result = Vec::with_capacity(len);

    let (left, right) = items.split_at_mut(len / 2);

    merge_sort(left);
    merge_sort(right);

    let mut l_iter = left.iter().peekable();
    let mut r_iter = right.iter().peekable();

    // It is safe to unwrap `next` when `peek` returns `Some`.
    while let (Some(l), Some(r)) = (l_iter.peek(), r_iter.peek()) {
        if l < r {
            result.push(*l_iter.next().unwrap());
        } else {
            result.push(*r_iter.next().unwrap());
        }
    }

    // Copy leftovers e.g. for `items` with value `[2, 3, 1]`, `left` is `[2, 3]`
    // and right is `[1]`, so `[3]` has not been accounted for in `result`.
    for l in l_iter {
        result.push(*l);
    }
    for r in r_iter {
        result.push(*r);
    }

    assert_eq!(len, result.len());

    // Copy the result to the original `items`.
    items[..len].copy_from_slice(&result[..len]);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let list1 = &mut [1, 3, 2, 11, 6, 8, 9, 2, 3, 1];
        bubble_sort(list1);
        assert_eq!(&mut [1, 1, 2, 2, 3, 3, 6, 8, 9, 11], list1);

        let list2 = &mut [1, 3, 2, 11, 6, 8, 9, -1, 2, 3, 1];
        bubble_sort(list2);
        assert_eq!(&mut [-1, 1, 1, 2, 2, 3, 3, 6, 8, 9, 11], list2);

        let list3 = &mut [1.01, 1.00, 10.5, 0.8, 0.001];
        bubble_sort(list3);
        assert_eq!(&mut [0.001, 0.8, 1.00, 1.01, 10.5], list3);

        let list4 = &mut ['a', 'c', 'b'];
        bubble_sort(list4);
        assert_eq!(&mut ['a', 'b', 'c'], list4);

        let list5 = &mut ["Test", "A old day", "A new day"];
        bubble_sort(list5);
        assert_eq!(&mut ["A new day", "A old day", "Test"], list5);
    }

    #[test]
    #[ignore]
    fn test_insertion_sort() {}

    #[test]
    fn test_merge_sort() {
        let list1 = &mut [1, 3, 2, 11, 6, 8, 9, 2, 3, 1];
        merge_sort(list1);
        assert_eq!(&mut [1, 1, 2, 2, 3, 3, 6, 8, 9, 11], list1);

        let list2 = &mut [1, 3, 2, 11, 6, 8, -1, 9, 2, 3, 1];
        merge_sort(list2);
        assert_eq!(&mut [-1, 1, 1, 2, 2, 3, 3, 6, 8, 9, 11], list2);

        let list3 = &mut [1.01, 1.00, 10.5, 0.8, 0.001];
        merge_sort(list3);
        assert_eq!(&mut [0.001, 0.8, 1.00, 1.01, 10.5], list3);

        let list4 = &mut ['a', 'c', 'b'];
        merge_sort(list4);
        assert_eq!(&mut ['a', 'b', 'c'], list4);

        let list5 = &mut ["Test", "A old day", "A new day"];
        merge_sort(list5);
        assert_eq!(&mut ["A new day", "A old day", "Test"], list5);
    }
}
