use crate::types::Vector1D;

use std::collections::HashMap;

pub fn mode(list: &Vector1D) -> Option<f64> {
    let mut occurrences = HashMap::new();
    for &number in list.iter() {
        *occurrences.entry(number as i64).or_insert(0) += 1;
    }
    occurrences
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(number, _)| number as f64)
}
#[cfg(test)]
use ndarray::prelude::*;
#[test]
fn mode_test() {
    let numbers: Vector1D = array![1.0, 2.0, 2.0, 3.0, 3.0, 3.0, 4.0, 4.0, 4.0, 4.0];
    let result = mode(&numbers);
    assert_eq!(result, Some(4.0));
}

#[test]
fn mode_test_single_element() {
    let numbers: Vector1D = array![1.0];
    let result = mode(&numbers);
    assert_eq!(result, Some(1.0));
}

#[test]
fn mode_test_no_repeats() {
    let numbers: Vector1D = array![1.0, 2.0, 3.0, 4.0];
    let result = mode(&numbers);
    assert!(result.is_some());
    assert!(
        result == Some(1.0) || result == Some(2.0) || result == Some(3.0) || result == Some(4.0)
    );
}

#[test]
fn mode_test_empty() {
    let numbers: Vector1D = array![];
    let result = mode(&numbers);
    assert_eq!(result, None);
}
