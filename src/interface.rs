//! # Interface
//!
//!

use crate::Rustunumic;

pub trait Interface<T> {
    /// Verify
    fn verify(&mut self, input: Vec<T>, target: Vec<T>) -> T;

    /// Query
    fn query(&mut self, input: Vec<T>) -> Vec<T>;

    /// Train
    fn train(&mut self, _input: Vec<T>, _target: Vec<T>) -> (usize, T);
}

impl<T> Interface<T> for Rustunumic<T> {
    fn verify(&mut self, _input: Vec<T>, _target: Vec<T>) -> T {
        let loss: T = todo!();
        loss
    }

    fn query(&mut self, _input: Vec<T>) -> Vec<T> {
        let output: Vec<T> = todo!();
        output
    }

    fn train(&mut self, _input: Vec<T>, _target: Vec<T>) -> (usize, T) {
        let count: usize = 0;
        let loss: f32 = todo!();
        (count, loss)
    }
}
