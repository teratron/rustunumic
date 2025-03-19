# Examples

```rust
fn set_hidden_layers<T>(params: &[T]) {}

fn main() {
    let mut rn = Rustunumic::<f32>::new();

    // Hidden layers
    rn.set_hidden_layers(&[
        // (neurons, activation, bias)
        (3, Activation::Sigmoid, true),
        (5, Activation::ReLU, true),
        (3, Activation::Sigmoid, false)
    ]);
    rn.set_hidden_layers_shape(&[3, 5, 3]);
    rn.set_hidden_layers_activation([Activation::Sigmoid, Activation::ReLU, Activation::Sigmoid]);
    rn.set_hidden_layers_bias([true, true, false]);

    // Output layers
    rn.set_output_layer(
        // neurons, activation, loss, bias
        3, Activation::Sigmoid, Loss::Tanh, false
    );
    rn.set_output_layer_activation(Activation::Sigmoid);
    rn.set_output_layer_loss(Loss::Tanh);
    rn.set_output_layer_bias(false);
}
```