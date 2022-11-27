use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let arr = array;
    let mid = arr.len() / 2;

    match key.cmp(arr.get(mid)?) {
        Ordering::Equal => Some(mid),
        Ordering::Greater => find(&arr[mid + 1..], key).map(|x| x + mid + 1),
        Ordering::Less => find(&arr[..mid], key),
    }
}
