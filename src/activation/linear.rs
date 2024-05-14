//! # Linear
//!
//!

use super::Float;

/// Linear/identity activation function.
///
/// # Arguments
///
/// * `value` - исходные данные.
/// * `slope` - определяет уровень наклона линии _(default 1.0)_.
/// * `offset` - определяет смещение линии от начала координат _(default 0.0)_.
///
/// # Returns
///
/// Возвращает взвешенную сумму исходных данных.
pub(super) fn activation<T: Float>(value: T, slope: f64, offset: f64) -> T {
    value * T::from(slope) + T::from(offset)
}

/// Linear/identity activation function derivative.
///
/// # Arguments
///
/// * `slope` - определяет уровень наклона линии _(default 1.0)_.
///
/// # Returns
///
/// Возвращает значение уровня наклона линии `slope`.
pub(super) fn derivative<T: Float>(slope: f64) -> T {
    T::from(slope)
}
