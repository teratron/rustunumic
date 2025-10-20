# Test-Driven Development (TDD) Workflow for Rust

## Brief Overview

This document is your guide to implementing and applying the Test-Driven Development (TDD) methodology in the `rustunumic` project. Adherence to these rules is mandatory for all team members. Our goal is not just to write code, but to build a reliable, maintainable, and efficient product. Every change must be justified and verified by tests.

## Core Principles

- **Test-First Approach**: All development—whether a new feature, a bug fix, or refactoring—begins with writing a test that reproduces the problem or describes the expected behavior. Production code is written only after a failing test has been created.
- **Isolate and Conquer**: If tests reveal failures in unrelated modules, create a separate, dedicated task to debug and fix that problem. Do not proceed with the original task until the blocking issue is resolved and covered by tests.
- **Mandatory Validation**: A task is considered complete only when all quality checks (linter, tests, formatting) pass successfully. Never assume a change works—rely only on the results of automated checks.
- **Meaningful Commits**: Follow the [Conventional Commits](https://www.conventionalcommits.org/) standard. Commit messages must be informative and reflect the essence of the changes.
  - `feat(network): implement backpropagation algorithm`
  - `fix(loss): correct calculation in MSE function`
  - `refactor(cell): simplify neuron state management`
  - `test(activation): add unit tests for ReLU function`

## Development & CI Workflow

The entire development and quality assurance process is built around the Cargo ecosystem.

- **Code Formatting**: `cargo fmt`
- **Quality Checks and Linting**: `cargo clippy -- -D warnings`
- **Running All Tests**: `cargo test`
- **Running Benchmarks**: `cargo bench`

These commands must be executed before every commit to ensure that only high-quality code is pushed to the repository.

## The Practice of Writing Tests in Rust

### Unit Tests

Unit tests are located in the same files as the code they are testing, inside a `#[cfg(test)]` module. They verify small, isolated pieces of logic.

**Example:**

```rust
// src/activation.rs

pub fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relu_positive_input() {
        assert_eq!(relu(5.0), 5.0);
    }

    #[test]
    fn test_relu_negative_input() {
        assert_eq!(relu(-5.0), 0.0);
    }

    #[test]
    fn test_relu_zero_input() {
        // Use assert! for floating-point comparisons
        assert!(relu(0.0) == 0.0, "ReLU of zero should be zero");
    }
}
```

### Integration Tests

Integration tests reside in the `tests/` directory and verify the interaction between multiple modules or the public API of your crate. They see your code as an external user would.

**Example (`tests/network_integration.rs`):**

```rust
// tests/network_integration.rs

use rustunumic::Rustunumic; // Import the crate

#[test]
fn test_network_creation_and_query() {
    let mut nn = Rustunumic::new();
    let inputs = vec![1.0, 2.0, 3.0];
    let result = nn.query(&inputs);

    // Assert that the result is Ok and has the expected length
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 1); // Example assertion
}
```

### Documentation Tests

Write usage examples directly in your doc-comments. `cargo test` will automatically run them and verify that they work. This ensures your documentation is always up-to-date.

**Example:**

```rust
/// Calculates the Mean Squared Error.
///
/// # Examples
///
/// ```
/// use rustunumic::loss::mse;
///
/// let target = vec![1.0, 2.0, 3.0];
/// let output = vec![1.2, 2.3, 2.8];
/// let error = mse(&target, &output);
/// assert!(error > 0.0);
/// ```
pub fn mse(target: &[f64], output: &[f64]) -> f64 {
    // ... implementation
    unimplemented!();
}
```

## Advanced Testing

### Performance Testing (Benchmarks)

To measure the performance of critical code sections, use benchmarks. They are placed in the `benches/` directory.

**Example (`benches/propagation_bench.rs`):**

```rust
// benches/propagation_bench.rs

#![feature(test)]

extern crate test;
use rustunumic::Rustunumic;
use test::Bencher;

#[bench]
fn bench_network_propagation(b: &mut Bencher) {
    let mut nn = Rustunumic::new();
    let inputs = vec![0.5; 100]; // 100 inputs

    b.iter(|| {
        // The code to be benchmarked
        nn.query(&inputs)
    });
}
```

### Isolating Dependencies with `mockall`

When you need to test a component in isolation from its dependencies (e.g., a database or a network service), use mocks. The `mockall` crate is an excellent tool for this.

**Example:**

```rust
// Cargo.toml
// [dev-dependencies]
// mockall = "0.11.0"

use mockall::automock;

#[automock]
pub trait DataProvider {
    fn fetch_data(&self) -> Result<Vec<f64>, String>;
}

pub struct Processor {
    provider: Box<dyn DataProvider>,
}

impl Processor {
    pub fn process(&self) -> f64 {
        let data = self.provider.fetch_data().unwrap_or_default();
        data.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_processor_with_mock() {
        let mut mock_provider = MockDataProvider::new();
        // Expect the method to be called once and return a predefined result
        mock_provider.expect_fetch_data()
            .times(1)
            .returning(|| Ok(vec![10.0, 20.0, 30.0]));

        let processor = Processor {
            provider: Box::new(mock_provider),
        };

        assert_eq!(processor.process(), 60.0);
    }
}
