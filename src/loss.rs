//! # Loss
//!
//!

use super::Float;

pub(super) const LOSS_LIMIT: f64 = 1E-10;

/// Loss mode
///
/// **Note:**
///
/// - MSE is mean squared error.
/// - RMSE is root mean squared error.
/// - Arctan is arctan error.
/// - Avg is average error.
#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Loss {
    /// Mean Squared Error.
    MSE,

    /// Root Mean Squared Error.
    RMSE,

    /// Arctan Error.
    Arctan,

    /// Average Error.
    Avg,
}

pub(super) fn get_total_loss<T: Float>(misses: Vec<T>, mode: &Loss) -> T {
    let mut loss = T::ZERO;
    let mut count = T::ZERO;
    misses.iter().for_each(|m| {
        loss += get_loss(m, mode);
        count += T::ONE;
    });
    if count > T::ONE {
        loss /= count;
    }
    if *mode == Loss::RMSE {
        loss = loss.float_sqrt();
    }
    loss
}

fn get_loss<T: Float>(value: &T, mode: &Loss) -> T {
    match mode {
        Loss::Avg => value.float_abs(),
        Loss::Arctan => value.float_atan().float_powi(2),
        Loss::MSE | Loss::RMSE => value.float_powi(2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_loss() {
        let value: f64 = 2.;

        assert_eq!(get_loss(&value, &Loss::MSE), 4.);
        assert_eq!(get_loss(&value, &Loss::RMSE), 4.);
        assert_eq!(get_loss(&value, &Loss::Avg), 2.);
        assert_eq!(get_loss(&value, &Loss::Arctan), value.atan().powi(2));
    }
}
