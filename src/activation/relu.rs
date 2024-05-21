//! # ReLU
//!
//!

use super::Float;

/// Rectified linear unit activation function.
///
/// # Arguments
///
/// * `value` - исходные данные.
/// * `leak` - определяет коэффициент "утечки" (пропуска отрицательных значений, _default 0.0_) .
///
/// # Returns
///
/// Возвращает взвешенную сумму исходных данных.
pub(super) fn activation<T: Float>(value: T, leak: f64) -> T {
    if value < T::ZERO {
        value * T::from(leak)
    } else {
        value
    }
}

/// Rectified linear unit activation function derivative.
///
/// # Arguments
///
/// * `value` - последнее состояние функции активации (результат прямого прохода).
/// * `leak` - определяет коэффициент "утечки" (пропуска отрицательных значений, _default 0.0_) .
///
/// # Returns
///
/// Возвращает значение коэффициент "утечки" `leak` или `1.0`.
pub(super) fn derivative<T: Float>(value: T, leak: f64) -> T {
    if value < T::ZERO {
        T::from(leak)
    } else {
        T::ONE
    }
}
