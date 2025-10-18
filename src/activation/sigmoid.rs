//! # Sigmoid
//!
//!

use super::Float;

/// Logistic, a.k.a. sigmoid or soft step activation function.
///
/// # Arguments
///
/// * `value` - исходные данные.
/// * `slope` - определяет уровень наклона линии в диапазоне от 0.0 до 1.0 _(default 1.0)_.
/// * `offset` - определяет смещение линии от начала координат _(default 0.0)_.
///
/// # Returns
///
/// Возвращает взвешенную сумму исходных данных.
pub(super) fn activation<T: Float>(value: T, slope: f64, offset: f64) -> T {
    //T::from(1.) / ((-value).float_exp() + T::ONE)
    T::from(slope) / ((-value).float_exp() + T::ONE - T::from(offset))
}

/// Logistic, a.k.a. sigmoid or soft step activation function derivative.
///
/// # Arguments
///
/// * `value` - последнее состояние функции активации (результат прямого прохода).
/// * `slope` - определяет уровень наклона линии в диапазоне от 0.0 до 1.0 _(default 1.0)_.
/// * `offset` - определяет смещение линии от начала координат _(default 0.0)_.
///
/// # Returns
///
/// Возвращает производную последнего состояния функции активации.
pub(super) fn derivative<T: Float>(value: T, slope: f64, offset: f64) -> T {
    //T::from(1.) - value
    let v = (value + T::from(offset)).float_min(slope).float_max(0.);
    v * (T::ONE - v / T::from(slope))
}
