use rayon::prelude::*;
use std::collections::LinkedList;

/// Helper for collecting parallel iterators to an intermediary
pub(crate) fn collect<I: IntoParallelIterator>(iter: I) -> LinkedList<Vec<I::Item>> {
    iter.into_par_iter()
        .fold(Vec::new, |mut vec, elem| {
            vec.push(elem);
            vec
        })
        .map(|vec| {
            let mut list = LinkedList::new();
            list.push_back(vec);
            list
        })
        .reduce(LinkedList::new, |mut list1, mut list2| {
            list1.append(&mut list2);
            list1
        })
}
