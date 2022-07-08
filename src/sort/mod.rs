//! # sort algorithms.
//!
//! `sort` defines various sorting algorithms on generic types.

/// Sorts in place using bubble sort.
///
/// Repeatedly step through the list, compare adjacent elements and swap them if
/// they are in the wrong order. Keep repeating the pass through until the list
/// is sorted. In practice this means there's a constant bubbling of the biggest
/// element from an unsorted lhs section to the top rhs section.
///
/// Worst-case performance: O(n^2) comparisons, O(n^2) swaps.
/// Best-case performance: O(n) comparisons, O(1) swaps.
/// Worst-case space complexity: O(n) total, O(1) auxiliary.
pub fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
    // for _ in 0..list.len() {
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

/// Sorts in place using insertion sort.
///
/// Iterate the list, and for each element, find the location it belongs and
/// transfer it to that location. Repeat until no input elements remain. In
/// practice this means there's a constant sinking of an element from an
/// unsorted section on the rhs into a sorted section on the lhs.
///
/// At each index, check the value against the largest value in the lhs. If
/// larger, leave the element in place and move on to the next element. If
/// smaller, shift all the larger elements and insert into that correct
/// position.
///
/// Insertion sort requires less comparisons and less swaps than bubble sort and
/// so is more efficient for large inputs.
///
/// E.g.
///
/// 5 1 2 6 3
///
/// BUBBLE
///
/// 1 5 2 6 3
/// 1 2 5 6 3
/// 1 2 5 6 3
/// 1 2 5 3 6
/// 1 2 5 3 6
/// 1 2 5 3 6
/// 1 2 3 5 6
/// 1 2 3 5 6
///
/// Comparison count    8
/// Swap count          4
///
/// INSERTION
///
/// 1 5 2 6 3
/// 1 2 5 6 3
/// 1 2 5 6 3
/// 1 2 5 6 3
/// 1 2 5 3 6
/// 1 2 3 5 6
/// 1 2 3 5 6
///
/// Comparison count    7
/// Swap count          3
///
/// Worst-case performance: O(n^2) comparisons, O(n^2) swaps.
/// Best-case performance: O(n) comparisons, O(1) swaps.
/// Worst-case space complexity: O(n) total, O(1) auxiliary.
pub fn insertion_sort<T: PartialOrd>(list: &mut [T]) {
    // Iterate for each element `i`, starting from index 1 since insertion sort
    // compares `i` to `i - 1`.
    for i in 1..list.len() {
        let mut j = i;
        // while j > 0 {
        //     if list[j - 1] > list[j] {
        //         ...
        //     } else {
        //         break;
        //     }
        while j > 0 && list[j - 1] > list[j] {
            list.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// TODO: Document algorithm.
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
    fn test_insertion_sort() {
        let list1 = &mut [1, 3, 2, 11, 6, 8, 9, 2, 3, 1];
        insertion_sort(list1);
        assert_eq!(&mut [1, 1, 2, 2, 3, 3, 6, 8, 9, 11], list1);

        let list2 = &mut [1, 3, 2, 11, 6, 8, 9, -1, 2, 3, 1];
        insertion_sort(list2);
        assert_eq!(&mut [-1, 1, 1, 2, 2, 3, 3, 6, 8, 9, 11], list2);

        let list3 = &mut [1.01, 1.00, 10.5, 0.8, 0.001];
        insertion_sort(list3);
        assert_eq!(&mut [0.001, 0.8, 1.00, 1.01, 10.5], list3);

        let list4 = &mut ['a', 'c', 'b'];
        insertion_sort(list4);
        assert_eq!(&mut ['a', 'b', 'c'], list4);

        let list5 = &mut ["Test", "A old day", "A new day"];
        insertion_sort(list5);
        assert_eq!(&mut ["A new day", "A old day", "Test"], list5);
    }

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
