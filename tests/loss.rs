//! Tests for the `loss` module.

use rustunumic::loss::{Loss, get_total_loss};

#[test]
fn test_loss_functions() {
    let value: f64 = 2.0;
    let epsilon = 1e-10; // Small tolerance for floating-point comparisons

    // Test MSE
    assert_eq!(rustunumic::loss::mse::calculate(&value), 4.0);

    // Test RMSE (component)
    assert_eq!(rustunumic::loss::rmse::calculate(&value), 4.0);

    // Test Avg (MAE)
    assert_eq!(rustunumic::loss::avg::calculate(&value), 2.0);
    assert_eq!(rustunumic::loss::mae::calculate(&value), 2.0);

    // Test Arctan
    let expected_arctan = value.atan().powi(2);
    let actual_arctan = rustunumic::loss::arctan::calculate(&value);
    assert!((actual_arctan - expected_arctan).abs() < epsilon);

    // Test BCE (placeholder)
    assert_eq!(rustunumic::loss::bce::calculate(&value), 0.0);

    // Test MAPE calculation
    // mape::calculate now computes the actual MAPE.
    // mape::calculate(&value, &1.0) computes (|1.0 - value| / |1.0|) * 100
    // For value = 2.0: (|1.0 - 2.0| / |1.0|) * 100 = (1.0 / 1.0) * 100 = 100.0
    assert_eq!(rustunumic::loss::mape::calculate(&value, &1.0), 100.0);

    // Test the calculate function (now computes MAPE)
    let predicted = 110.0_f64;
    let target = 100.0_f64;
    let expected_mape = 10.0; // (|100 - 110| / |100|) * 100 = (10/100)*100 = 10%
    let actual_mape = rustunumic::loss::mape::calculate(&predicted, &target);
    assert!((actual_mape - expected_mape).abs() < epsilon);

    // Test MAPE with zero target
    let predicted_zero = 1.0_f64;
    let target_zero = 0.0_f64;
    let expected_mape_zero = 0.0; // Should return 0.0 to avoid division by zero
    let actual_mape_zero = rustunumic::loss::mape::calculate(&predicted_zero, &target_zero);
    assert_eq!(actual_mape_zero, expected_mape_zero);
}

#[test]
fn test_get_total_loss() {
    let misses: Vec<f64> = vec![1.0, 2.0, 3.0];
    let epsilon = 1e-10;

    // Test MSE
    let expected_mse = (1.0_f64 + 4.0 + 9.0) / 3.0; // (1^2 + 2^2 + 3^2) / 3
    let actual_mse = get_total_loss(misses.iter().collect(), &Loss::MSE);
    assert!((actual_mse - expected_mse).abs() < epsilon);

    // Test RMSE
    let expected_rmse = expected_mse.sqrt(); // sqrt of MSE
    let actual_rmse = get_total_loss(misses.iter().collect(), &Loss::RMSE);
    assert!((actual_rmse - expected_rmse).abs() < epsilon);

    // Test Avg (MAE)
    let expected_avg = (1.0_f64 + 2.0 + 3.0) / 3.0; // (|1| + |2| + |3|) / 3
    let actual_avg = get_total_loss(misses.iter().collect(), &Loss::Avg);
    assert!((actual_avg - expected_avg).abs() < epsilon);

    let actual_mae = get_total_loss(misses.iter().collect(), &Loss::MAE);
    assert!((actual_mae - expected_avg).abs() < epsilon);

    // Test Arctan
    let expected_arctan =
        (1.0_f64.atan().powi(2) + 2.0_f64.atan().powi(2) + 3.0_f64.atan().powi(2)) / 3.0;
    let actual_arctan = get_total_loss(misses.iter().collect(), &Loss::Arctan);
    assert!((actual_arctan - expected_arctan).abs() < epsilon);

    // Test BCE (placeholder)
    let expected_bce = 0.0; // Placeholder returns 0
    let actual_bce = get_total_loss(misses.iter().collect(), &Loss::BCE);
    assert!((actual_bce - expected_bce).abs() < epsilon);

    // Test MAPE (placeholder)
    // get_total_loss for MAPE calls mape::calculate(value, &T::ONE)
    // For misses = [1.0, 2.0, 3.0]:
    // mape::calculate(&1.0, &1.0) = (|1.0 - 1.0| / |1.0|) * 100 = (0.0 / 1.0) * 100 = 0.0
    // mape::calculate(&2.0, &1.0) = (|1.0 - 2.0| / |1.0|) * 100 = (1.0 / 1.0) * 100 = 100.0
    // mape::calculate(&3.0, &1.0) = (|1.0 - 3.0| / |1.0|) * 100 = (2.0 / 1.0) * 100 = 200.0
    // Average = (0.0 + 100.0 + 200.0) / 3 = 300.0 / 3 = 100.0
    let expected_mape = 100.0;
    let actual_mape = get_total_loss(misses.iter().collect(), &Loss::MAPE);
    assert!((actual_mape - expected_mape).abs() < epsilon);
}
