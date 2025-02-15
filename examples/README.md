# Examples

```rust
fn set_hidden_layers<T>(params: &[T]) {}

fn main() {
    // Hidden layers
    set_hidden_layers(&[
        // (neurons, activation, bias)
        (3, Activation::Sigmoid, true),
        (5, Activation::ReLU, true),
        (3, Activation::Sigmoid, false)
    ]);
    set_hidden_layers_shape(&[3, 5, 3]);
    set_hidden_layers_activation([Activation::Sigmoid, Activation::ReLU, Activation::Sigmoid]);
    set_hidden_layers_bias([true, true, false]);

    // Output layers
    set_output_layer(
        // neurons, activation, loss, bias
        3, Activation::Sigmoid, Loss::Tanh, false
    );
    set_output_layer_activation(Activation::Sigmoid);
    set_output_layer_loss(Loss::Tanh);
    set_output_layer_bias(false);
}
```