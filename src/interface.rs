//! # Interface
//!
//!

use crate::Rustunumic;

pub trait Interface {
    /// Verify
    fn verify(&mut self, input: Vec<f32>, target: Vec<f32>) -> f32;

    /// Query
    fn query(&mut self, input: Vec<f32>) -> Vec<f32>;

    /// Train
    fn train(&mut self, _input: Vec<f32>, _target: Vec<f32>) -> (usize, f32);
}

impl Interface for Rustunumic {
    fn verify(&mut self, _input: Vec<f32>, _target: Vec<f32>) -> f32 {
        let loss: f32 = todo!();
        loss
    }

    fn query(&mut self, _input: Vec<f32>) -> Vec<f32> {
        let output: Vec<f32> = todo!();
        output
    }

    fn train(&mut self, _input: Vec<f32>, _target: Vec<f32>) -> (usize, f32) {
        let count: usize = 0;
        let loss: f32 = todo!();
        (count, loss)
    }
}