# Rustunumic

![Crates.io License (version)](https://img.shields.io/crates/l/rustunumic/0.1.0?style=flat)
![docs.rs (with version)](https://img.shields.io/docsrs/rustunumic/0.1.0?style=flat&logo=docs.rs)
![Crates.io Total Downloads](https://img.shields.io/crates/d/rustunumic?style=flat&logo=rust)
![Crates.io Version](https://img.shields.io/crates/v/rustunumic?style=flat&logo=rust&label=rustunumic)
[![fork with dotenv-vault](https://badge.dotenv.org/fork.svg?r=1)](https://vault.dotenv.org/project/vlt_ea1a620a9e0e1bad96488e86eff7a82a82b6eef20c8d40c0ae0bac94029e6191/example)

---

## Description

Simple neural network library for Rust.

## Visuals

## Installation

```shell
cargo add rustunumic
```

If you have already added **rustunumic**, you can update to the latest version by using:

```shell
cargo update rustunumic
```

## Usage

```rust
use rustunumic::Rustunumic;

fn main() {
    // Returns a new neural network instance.
    let mut rn = Rustunumic::<f32>::new();

    // Dataset.
    data_input = [0.27, 0.31];
    data_target = [0.7];

    // Training dataset.
    let (num, loss) = rn.train(&data_input, &data_target);
    print!("{num:?} {loss:?}");
}
```

## Documentation

### Properties of Neural Network

#### _bias_

The neuron bias, false or true (required field for a config).

#### _hidden_layers_

Array of the number of neurons in each hidden layers.

#### _activation_mode_

ActivationMode function mode (required field for a config).

| Code | Activation | Description                           |
|:----:|:-----------|:--------------------------------------|
|  0   | Linear     | Linear/identity                       |
|  1   | ReLU       | Rectified Linear Unit                 |
|  2   | LeakyReLU  | Leaky Rectified Linear Unit           |
|  3   | Sigmoid    | Logistic, a.k.a. sigmoid or soft step |
|  4   | TanH       | Hyperbolic Tangent                    |

#### _loss_mode_

The mode of calculation of the total error.

| Code | Loss   | Description             |
|:----:|:-------|:------------------------|
|  0   | MSE    | Mean Squared Error      |
|  1   | RMSE   | Root Mean Squared Error |
|  2   | Arctan | Arctan Error            |
|  3   | Avg    | Average Error           |

#### _loss_limit_

Minimum (sufficient) limit of the average of the error during training.

#### _rate_

Learning coefficient (greater than 0.0 and less than or equal to 1.0).

More documentation is available at the [rustunumic website](https://teratron.github.io/rustunumic).

## Examples

You can find examples of neural networks in the [example's directory](examples).

- [perceptron](examples/perceptron)
- [linear](examples/linear)
- [query](examples/query)
- [verify](examples/verify)
- [and_train](examples/and_train)

## Support

## Roadmap

## Contributing

## Authors and acknowledgment

## License

[MIT License](LICENSE).

## Project status

_Project at the initial stage._

See the latest [commits](https://github.com/teratron/rustunumic/commits/master).

---
