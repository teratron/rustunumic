//! # Interface
//!
//!

use crate::{float::Float, Rustunumic};

pub trait Interface<T> {
    /// Verify
    fn verify(&mut self, input: Vec<T>, target: Vec<T>) -> T;

    /// Query
    fn query(&mut self, input: Vec<T>) -> Vec<T>;

    /// Train
    fn train(&mut self, _input: Vec<T>, _target: Vec<T>) -> (usize, T);
}

impl<T: Float> Interface<T> for Rustunumic<T> {
    fn verify(&mut self, _input: Vec<T>, _target: Vec<T>) -> T {
        let loss: T = T::ZERO;
        loss
    }

    fn query(&mut self, _input: Vec<T>) -> Vec<T> {
        let output: Vec<T> = vec![T::ZERO; 10];
        output
    }

    fn train(&mut self, _input: Vec<T>, _target: Vec<T>) -> (usize, T) {
        let count: usize = 0;
        let loss: T = T::ZERO;
        (count, loss)
    }
}
