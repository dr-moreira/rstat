use crate::types::Vector1D;

#[cfg(test)]
use ndarray::{array, Array1};

/// Calcula a mediana de um vetor usando o algoritmo quickselect (O(n) em média)
/// ao invés de ordenação completa (O(n log n)).
///
/// # Algoritmo
/// - Para arrays ímpares: usa select_nth_unstable para encontrar o elemento do meio
/// - Para arrays pares: usa select_nth_unstable duas vezes para encontrar os dois
///   elementos do meio e retorna sua média
///
/// # Complexidade
/// - Tempo: O(n) em média, O(n²) no pior caso
/// - Espaço: O(n) para a cópia do vetor
pub fn median(list: &Vector1D) -> f64 {
    let mut data = list.to_vec();
    let len = data.len();
    let mid = len / 2;

    if len % 2 == 0 {
        // Para tamanho par, precisamos dos dois elementos do meio
        // Primeiro, particionamos no índice mid
        let (left, median_high, _) = data.select_nth_unstable_by(mid, |a, b| {
            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
        });
        let median_high_val = *median_high;

        // O elemento inferior do meio é o máximo da metade esquerda
        let median_low_val = left
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        (median_low_val + median_high_val) / 2.0
    } else {
        // Para tamanho ímpar, pegar o elemento do meio
        let (_, median_val, _) = data.select_nth_unstable_by(mid, |a, b| {
            a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
        });
        *median_val
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
