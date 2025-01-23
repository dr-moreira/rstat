use crate::deviation::{population_standard_deviation, sample_standard_deviation};
use crate::types::Vector1D;
use crate::variance::{population_covariance, sample_covariance};

pub fn sample_correlation(list1: &Vector1D, list2: &Vector1D) -> f64 {
    let cov = sample_covariance(list1, list2);
    let std_dev1 = sample_standard_deviation(list1);
    let std_dev2 = sample_standard_deviation(list2);
    cov / (std_dev1 * std_dev2)
}

pub fn population_correlation(list1: &Vector1D, list2: &Vector1D) -> f64 {
    let cov = population_covariance(list1, list2);
    let std_dev1 = population_standard_deviation(list1);
    let std_dev2 = population_standard_deviation(list2);
    cov / (std_dev1 * std_dev2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn sample_correlation_test() {
        let list1: Vector1D = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let list2: Vector1D = array![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = sample_correlation(&list1, &list2);
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn population_correlation_test() {
        let list1: Vector1D = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let list2: Vector1D = array![2.0, 4.0, 6.0, 8.0, 10.0];
        let result = population_correlation(&list1, &list2);
        assert!((result - 1.0).abs() < 1e-10);
    }
}
