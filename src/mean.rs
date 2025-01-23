use std::panic;

use crate::types::Vector1D;

fn check_for_zero(vector: &Vector1D) -> bool {
    vector.iter().any(|&x| x == 0.0)
}

pub fn harmonic_mean(list: &Vector1D) -> f64 {
    if list.is_any_nan() || check_for_zero(list) {
        panic!("Cannot calculate harmonic mean NAN or 0.0 on list");
    }
    let sum = list.mapv(|x| 1.0 / x).sum();
    let l = list.len();

    l as f64 / sum
}

pub fn mean(list: &Vector1D) -> f64 {
    if let Some(mean) = list.mean() {
        return mean;
    } else {
        panic!("Cannot calculate mean of an empty list");
    }
}

pub fn geometric_mean(list: &Vector1D) -> f64 {
    if list.is_any_nan() || check_for_zero(list) {
        panic!("Cannot calculate geometric mean of an empty list");
    }
    let ln_sum = list.ln().sum();
    let l = list.len();

    (ln_sum / l as f64).exp()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::{array, Array1};
    use std::f64::NAN;

    #[should_panic]
    #[test]
    fn harmonic_mean_nan_panic() {
        let numbers: Array1<f64> = array![5.0, 6.0, NAN, 8.0];
        let _result = harmonic_mean(&numbers);
    }

    #[should_panic]
    #[test]
    fn harmonic_mean_0_panic() {
        let numbers: Array1<f64> = array![5.0, 6.0, 0.0, 8.0];
        let _result = harmonic_mean(&numbers);
    }
    #[test]
    fn harmonic_mean_large_1d() {
        let numbers: Array1<f64> = array![
            5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0,
            20.0, 21.0, 22.0, 23.0, 24.0, 25.0
        ];
        let result = harmonic_mean(&numbers);
        assert_eq!(result, 12.120338726314232_f64);
    }
    #[test]
    fn geometric_mean_large_1d() {
        let numbers: Array1<f64> = array![
            5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0,
            20.0, 21.0, 22.0, 23.0, 24.0, 25.0
        ];
        let result = geometric_mean(&numbers);
        assert_eq!(result, 13.609125256834382_f64);
    }
    #[test]
    #[should_panic]
    fn geometric_mean_0_panic() {
        let numbers: Array1<f64> = array![1.0, 0.0, 8.0, 16.0];
        let _result = geometric_mean(&numbers);
    }
    #[test]
    fn geometric_mean_1d() {
        let numbers: Array1<f64> = array![1.0, 2.0, 8.0, 16.0];
        let result = geometric_mean(&numbers);
        assert_eq!(result, 4.0);
    }
    #[test]
    #[should_panic(expected = "Cannot calculate mean of an empty list")]
    fn mean_empty_list() {
        let numbers: Array1<f64> = array![];
        let result = mean(&numbers);
        assert_eq!(result, 3.0);
    }
    #[test]
    fn mean_1d() {
        let numbers: Array1<f64> = array![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = mean(&numbers);
        assert_eq!(result, 3.0);
    }
}
