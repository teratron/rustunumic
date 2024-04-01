# Rustunumic

[![fork with dotenv-vault](https://badge.dotenv.org/fork.svg?r=1)](https://vault.dotenv.org/project/vlt_ea1a620a9e0e1bad96488e86eff7a82a82b6eef20c8d40c0ae0bac94029e6191/example)

---

## Description

Simple neural network library for rust.

## Visuals



## Installation

```shell
cargo add rustunumic
```

## Usage

```rust
use rustunumic::Rustunumic;

fn main() {
    // Returns a new neural network instance.
    let mut rn = Rustunumic::new();

    // Dataset.
    data_input  = [0.27, 0.31]
    data_target = [0.7]

    // Training dataset.
    let (num, loss) = rn.train(&data_input, &data_target);
}
```

## Documentation

### Properties of Neural Network

#### _name_

Neural network architecture name (required field for a config).

#### _bias_

The neuron bias, false or true (required field for a config).

#### _hidden_layer_

Array of the number of neurons in each hidden layer.

#### _activation_mode_

ActivationMode function mode (required field for a config).

| Activation | Description                              |
|------------|------------------------------------------|
| LINEAR     | Linear/identity                          |
| RELU       | ReLu (rectified linear unit)             |
| LEAKY_RELU | Leaky ReLu (leaky rectified linear unit) |
| SIGMOID    | Logistic, a.k.a. sigmoid or soft step    |
| TANH       | TanH (hyperbolic tangent)                |

#### _loss_mode_

The mode of calculation of the total error.

| Loss   | Description             |
|--------|-------------------------|
| MSE    | Mean Squared Error      |
| RMSE   | Root Mean Squared Error |
| ARCTAN | Arctan                  |
| AVG    | Average                 |

#### _loss_limit_

Minimum (sufficient) limit of the average of the error during training.

#### _rate_

Learning coefficient (greater than 0.0 and less than or equal to 1.0).

More documentation is available at the [rustunumic website](https://teratron.github.io/rustunumic).

---

## Examples

You can find examples of neural networks in the [example's directory](examples).

- [perceptron](examples/perceptron)
- [linear](examples/linear)
- [query](examples/query)
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

![My Skills](https://skillicons.dev/icons?i=python,golang,rust,git,github)
