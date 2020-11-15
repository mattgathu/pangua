use std::cmp::Ord;

pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord;
}

// fancy approach: extend slice to have a sort_by_sorter method
pub fn sort<T, S>(slice: &mut [T], sorter: S)
where
    T: Ord,
    S: Sorter,
{
    sorter.sort(slice)
}

/// Bubble Sort
///
/// Bubble sort, sometimes referred to as sinking sort, is a simple sorting algorithm
/// that repeatedly steps through the list, compares adjacent elements and swaps them
/// if they are in the wrong order.
pub struct BubbleSort;

impl Sorter for BubbleSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i] < slice[i - 1] {
                    slice.swap(i, i - 1);
                    swapped = true;
                }
            }
        }
    }
}

/// Insertion Sort
///
/// Insertion sort iterates, consuming one input element each repetition, and growing a
/// sorted output list. At each iteration, insertion sort removes one element from the
/// input data, finds the location it belongs within the sorted list, and inserts it there.
/// It repeats until no input elements remain.
pub struct InsertionSort {
    pub smart: bool,
}
impl Sorter for InsertionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | not sorted]
        for unsorted in 1..slice.len() {
            if !self.smart {
                let mut i = unsorted;
                while i > 0 && slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    i -= 1;
                }
            } else {
                // use binary search to find index
                let i = match slice[..unsorted].binary_search(&slice[unsorted]) {
                    Ok(i) | Err(i) => i,
                };
                slice[i..=unsorted].rotate_right(1)
            }
        }
    }
}

/// Selection Sort
///
/// The algorithm divides the input list into two parts: a sorted sublist of items which
/// is built up from left to right at the front (left) of the list and a sublist of the
/// remaining unsorted items that occupy the rest of the list. Initially, the sorted
/// sublist is empty and the unsorted sublist is the entire input list.
/// The algorithm proceeds by finding the smallest (or largest, depending on sorting order)
/// element in the unsorted sublist, exchanging (swapping) it with the leftmost unsorted
/// element (putting it in sorted order), and moving the sublist boundaries one element to the right.
///
/// The time efficiency of selection sort is quadratic, so there are a number of sorting techniques
/// which have better time complexity than selection sort. One thing which distinguishes selection
/// sort from other sorting algorithms is that it makes the minimum possible number of swaps, n − 1 in the worst case.
pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | not sorted]
        for unsorted in 0..slice.len() {
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|t| t.1) // min value
                .map(|t| unsorted + t.0) // get index
                .expect("slice is non-empty");
            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest)
            }
        }
    }
}

/// Quick Sort
///
/// Quicksort is a divide-and-conquer algorithm. It works by selecting a 'pivot' element
/// from the array and partitioning the other elements into two sub-arrays, according
/// to whether they are less than or greater than the pivot. The sub-arrays are then
/// sorted recursively. This can be done in-place, requiring small additional amounts
/// of memory to perform the sorting.
pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1)
            }
            return;
        }
        _ => {}
    }
    let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");
    let mut left = 0;
    let mut right = rest.len() - 1;
    while left <= right {
        if &rest[left] <= pivot {
            left += 1;
        } else if &rest[right] > pivot {
            // avoid unnecessary swaps
            // we must be done
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            // move element to the right side
            rest.swap(left, right);
            left += 1;
            // we must be done
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }

    // place pivot in final position
    slice.swap(0, left);
    let (left, right) = slice.split_at_mut(left);
    assert!(left.last() <= right.first());
    quicksort(left);
    quicksort(&mut right[1..])
}
impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [ unsorted | pivot | unsorted ]
        quicksort(slice)
    }
}

pub struct StdSorter;
impl Sorter for StdSorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        slice.sort()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn std_works() {
        let mut tings = vec![5, 1, 4, 2, 3];
        StdSorter.sort(&mut tings);
        assert_eq!(tings, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn bubble_works() {
        let mut tings = vec![5, 1, 4, 2, 3];
        BubbleSort.sort(&mut tings);
        assert_eq!(tings, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn insertion_dumb_works() {
        let mut tings = vec![5, 1, 4, 2, 3];
        InsertionSort { smart: false }.sort(&mut tings);
        assert_eq!(tings, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn insertion_smart_works() {
        let mut tings = vec![5, 1, 4, 2, 3];
        InsertionSort { smart: true }.sort(&mut tings);
        assert_eq!(tings, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn selection_works() {
        let mut tings = vec![5, 1, 4, 2, 3];
        SelectionSort.sort(&mut tings);
        assert_eq!(tings, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn quick_works() {
        let mut tings = vec![5, 1, 4, 2, 3];
        QuickSort.sort(&mut tings);
        assert_eq!(tings, &[1, 2, 3, 4, 5]);
    }
}
