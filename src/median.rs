use crate::{sort::PermuteArray, sort::SortArray, types::Vector1D};
use ndarray::prelude::*;

pub fn median(list: &Vector1D) -> f64 {
    let list = list.clone();
    let l = list.len();
    let perm = list.sort_axis_by(Axis(0), |i, j| list[[i]] > list[[j]]);
    let b = list.permute_axis(Axis(0), &perm);

    let mid = l / 2;
    if l % 2 == 0 {
        (b[mid - 1] + b[mid]) / 2.0
    } else {
        b[mid]
    }
}

#[cfg(test)]
#[test]
fn median_test_odd() {
    let numbers: Array1<f64> = array![5.0, 6.0, 7.0, 8.0, 9.0];
    let result = median(&numbers);
    assert_eq!(result, 7.0_f64);
}

#[test]
fn median_test_even() {
    let numbers: Array1<f64> = array![
        24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0,
        5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0,
        21.0, 22.0, 23.0, 24.0
    ];
    let result = median(&numbers);
    assert_eq!(result, 16.0_f64);
}
