use crate::mean::mean;
use crate::types::Vector1D;

fn covariance(list1: &Vector1D, list2: &Vector1D, sample: bool) -> f64 {
    let mean1 = mean(list1);
    let mean2 = mean(list2);
    let mut l = list1.len();
    let l2 = list2.len();
    if l != l2 || l == 0 || l2 == 0 {
        panic!("The two lists must have the same length and different from zero")
    }
    let mut sum = 0.0;
    for (x, y) in list1.iter().zip(list2.iter()) {
        sum += (x - mean1) * (y - mean2);
    }
    if sample {
        l -= 1;
    }
    sum / l as f64
}

pub fn sample_covariance(list1: &Vector1D, list2: &Vector1D) -> f64 {
    covariance(list1, list2, true)
}

pub fn population_covariance(list1: &Vector1D, list2: &Vector1D) -> f64 {
    covariance(list1, list2, false)
}

fn variance(list: &Vector1D, sample: bool) -> f64 {
    let mean = mean(list);
    let mut l = list.len();
    let sum = list.mapv(|x| (x - mean).powi(2)).sum();
    if sample {
        l -= 1;
    }
    sum / l as f64
}
pub fn sample_variance(list: &Vector1D) -> f64 {
    variance(list, true)
}

pub fn population_variance(list: &Vector1D) -> f64 {
    variance(list, false)
}

#[cfg(test)]
use ndarray::prelude::*;

#[test]
fn population_variance_test() {
    let numbers: Vector1D = array![
        24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0,
        5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0,
        21.0, 22.0, 23.0, 24.0
    ];
    let result = population_variance(&numbers);
    assert_eq!(result, 28.53061224489796_f64);
}
#[test]
fn sample_variance_test() {
    let numbers: Vector1D = array![
        24.0, 23.0, 22.0, 21.0, 20.0, 19.0, 18.0, 17.0, 16.0, 15.0, 14.0, 13.0, 12.0, 11.0, 10.0,
        5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0, 20.0,
        21.0, 22.0, 23.0, 24.0
    ];
    let result = sample_variance(&numbers);
    assert_eq!(result, 29.369747899159663_f64);
}

#[test]
fn sample_covariance_test() {
    let numbers1: Vector1D = array![5.0, 6.0, 7.0, 8.0, 9.0];
    let numbers2: Vector1D = array![5.0, 6.0, 7.0, 8.0, 9.0];
    let result = sample_covariance(&numbers1, &numbers2);
    assert_eq!(result, 2.5_f64);
}

#[test]
fn population_covariance_test() {
    let numbers1: Vector1D = array![5.0, 6.0, 7.0, 8.0, 9.0];
    let numbers2: Vector1D = array![5.0, 6.0, 7.0, 8.0, 9.0];
    let result = population_covariance(&numbers1, &numbers2);

    assert_eq!(result, 2.0_f64);
}
