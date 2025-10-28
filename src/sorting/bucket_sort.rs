///  **Bucket Sort**
///
/// From [Wikipedia](https://en.wikipedia.org/wiki/Bucket_sort):
/// Bucket sort, or bin sort, is a sorting algorithm that works by distributing
/// elements into several groups called *buckets*. Each bucket is then sorted individually,
/// either using a different sorting algorithm (like insertion sort) or by recursively
/// applying the bucket sort itself.
///
/// ---
///
/// ### How It Works
/// 1. **Setup:** Create an array of initially empty buckets.
/// 2. **Scatter:** Distribute elements from the input array into buckets.
/// 3. **Sort:** Sort each non-empty bucket (e.g., using insertion sort).
/// 4. **Gather:** Combine all buckets in order to form the final sorted array.
///
/// ---
///
/// ###  Complexity
/// - **Best Case:** `O(n + k)`
/// - **Average Case:** `O(n + k)`
/// - **Worst Case:** `O(n²)` (when all elements fall into one bucket)
/// - **Space Complexity:** `O(n + k)`
///
/// ---
///
/// ###  When to Use
/// - When input data is *uniformly distributed* over a known range.
/// - Works best with floating-point or bounded integer data.
///
/// ---
///
/// ###  References
/// - [Wikipedia](https://en.wikipedia.org/wiki/Bucket_sort)
/// - [GeeksforGeeks](https://www.geeksforgeeks.org/bucket-sort/)
/// - [TutorialsPoint](https://www.tutorialspoint.com/bucket-sort-in-data-structure)
///
use crate::sorting::insertion_sort::InsertionSort;
use crate::sorting::traits::Sorter;

fn bucket_sort<T: Ord + Copy + Into<usize>>(arr: &mut [T]) {
      if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap();
    let len = arr.len();
    let mut buckets = vec![vec![]; len + 1];

    for x in arr.iter() {
        buckets[len * (*x).into() / max.into()].push(*x);
    }

    for bucket in buckets.iter_mut() {
        InsertionSort::sort_inplace(bucket);
    }

    let mut i = 0;
    for bucket in buckets {
        for x in bucket {
            arr[i] = x;
            i += 1;
        }
    }
}

/// Sort a slice using bucket sort algorithm.
///
/// Time complexity is `O(n + k)` on average, where `n` is the number of elements,
/// `k` is the number of buckets used in process.
///
/// Space complexity is `O(n + k)`, as it sorts not in-place.
pub struct BucketSort;

impl<T> Sorter<T> for BucketSort
where
    T: Ord + Copy + Into<usize>,
{
    fn sort_inplace(arr: &mut [T]) {
        bucket_sort(arr);
    }
}
