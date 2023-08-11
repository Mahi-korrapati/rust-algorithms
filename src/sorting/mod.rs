include!("macros.rs");

pub fn is_sorted<T: fmt::Debug>(iterator: impl IntoIterator<Item = T>) -> bool
where
    T: PartialOrd,
{
    let vector: Vec<T> = iterator.into_iter().collect();

    if vector.is_empty() {
        return true;
    }

    for i in 0..vector.len() - 1 {
        if vector[i + 1] < vector[i] {
            println!("This vector is not sorted: {:#?}", vector);
            return false;
        }
    }

    true
}

mod bogo_sort;
mod bubble_sort;
mod bucket_sort;
mod cocktail_shaker_sort;
mod comb_sort;
mod counting_sort;
mod cycle_sort;
mod exchange_sort;
mod gnome_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod odd_even_sort;
mod pancake_sort;
mod quick_sort;
mod radix_sort;
mod selection_sort;
mod shell_sort;
mod stooge_sort;
mod tim_sort;
mod traits;

use std::fmt;

pub use self::bogo_sort::BogoSort;
pub use self::bubble_sort::BubbleSort;
pub use self::bucket_sort::BucketSort;
pub use self::cocktail_shaker_sort::CocktailShakerSort;
pub use self::comb_sort::CombSort;
pub use self::counting_sort::CountingSort;
pub use self::cycle_sort::CycleSort;
pub use self::exchange_sort::ExchangeSort;
pub use self::gnome_sort::GnomeSort;
pub use self::heap_sort::HeapSort;
pub use self::insertion_sort::InsertionSort;
pub use self::merge_sort::MergeSort;
pub use self::odd_even_sort::OddEvenSort;
pub use self::pancake_sort::PancakeSort;
pub use self::quick_sort::QuickSort;
pub use self::radix_sort::RadixSort;
pub use self::selection_sort::SelectionSort;
pub use self::shell_sort::ShellSort;
pub use self::stooge_sort::StoogeSort;
pub use self::tim_sort::TimSort;

#[cfg(test)]
mod tests {
    #[test]
    fn is_sorted() {
        assert_sorted!(&[] as &[isize]);
        assert_sorted!(&["a"]);
        assert_sorted!(&[1, 2, 3]);
        assert_sorted!(&[0, 1, 1]);

        assert_not_sorted!(&[1, 0]);
        assert_not_sorted!(&[2, 3, 1, -1, 5]);
    }
}
