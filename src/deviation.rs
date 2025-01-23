use crate::{
    types::Vector1D,
    variance::{population_variance, sample_variance},
};

pub fn sample_standard_deviation(list: &Vector1D) -> f64 {
    sample_variance(list).sqrt()
}

pub fn population_standard_deviation(list: &Vector1D) -> f64 {
    population_variance(list).sqrt()
}

#[cfg(test)]
use ndarray::array;
#[test]
fn sample_standard_deviation_test() {
    let numbers: Vector1D = array![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = sample_standard_deviation(&numbers);
    assert!((result - 1.5811388300841898).abs() < 1e-10);
}

#[test]
fn population_standard_deviation_test() {
    let numbers: Vector1D = array![1.0, 2.0, 3.0, 4.0, 5.0];
    let result = population_standard_deviation(&numbers);
    assert!((result - 1.4142135623730951).abs() < 1e-10);
}
