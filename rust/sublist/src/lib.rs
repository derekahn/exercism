use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn contains<T: PartialEq>(a: &[T], b: &[T]) -> bool {
    b.is_empty() || a.windows(b.len()).any(|section| section == b)
}

pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    use Comparison::*;

    match first_list.len().cmp(&second_list.len()) {
        Ordering::Equal => {
            if first_list == second_list {
                return Equal;
            }
        }
        Ordering::Greater => {
            if contains(first_list, second_list) {
                return Superlist;
            }
        }
        Ordering::Less => {
            if contains(second_list, first_list) {
                return Sublist;
            }
        }
    }

    return Unequal;
}
