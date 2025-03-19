//! # Properties
//!
//!

use crate::{Activation, Float, Loss, Rustunumic};

impl<'a, T: Float> Rustunumic<'a, T> {
    /// Set bias.
    pub fn set_bias(&mut self, bias: bool) -> &mut Self {
        match bias {
            true => self.bias = Some(bias),
            false => self.bias = None,
        }
        self
    }

    /// Set learning rate.
    pub fn set_rate(&mut self, rate: T) -> &mut Self {
        self.rate = rate;
        self
    }

    /// Set loss mode.
    pub fn set_loss_mode(&mut self, loss_mode: Loss) -> &mut Self {
        self.loss_mode = loss_mode;
        self
    }

    /// Set activation mode.
    pub fn set_activation_mode(&mut self, activation_mode: Activation) -> &mut Self {
        self.activation_mode = Some(activation_mode);
        self
    }

    /// Set hidden layers.
    pub fn set_hidden_layers(&mut self, layers: &[(usize, Activation, bool)]) -> &mut Self {
        //println!("hidden_layers: {hidden_layers:?}");
        /*let n = hidden_layers.iter().sum::<usize>();
        self.network.hidden.set_number(n);
        self.network.hidden.cells = (0..n)
            //.map(|_| Box::new(HiddenCell::new(Activation::Linear)) as Box<dyn Neuron<T>>)
            .map(|_| HiddenCell::new(Activation::Linear))
            .collect();*/

        //self.hidden_layers = hidden_layers;
        self
    }

    /// Set hidden layers shape.
    pub fn set_hidden_layers_shape(&mut self, nums: &[usize]) -> &mut Self {
        // let n = hidden_layers.iter().sum::<usize>();
        // self.network.hidden.set_number(n);
        // self.network.hidden.cells = (0..n)
        //     //.map(|_| Box::new(HiddenCell::new(Activation::Linear)) as Box<dyn Neuron<T>>)
        //     .map(|_| HiddenCell::new(Activation::Linear))
        //     .collect();

        self
    }

    /// Set hidden layers activation.
    pub fn set_hidden_layers_activation(&mut self, activations: &[Activation]) -> &mut Self {
        //self.network.hidden.cells.iter_mut().for_each(|n| n.set_activation(activation));
        self
    }

    /// Set hidden layers bias.
    pub fn set_hidden_layers_bias(&mut self, bias: &[bool]) -> &mut Self {
        //self.network.hidden.cells.iter_mut().for_each(|n| n.set_bias(bias));
        self
    }

    /// Set output layer.
    pub fn set_output_layer(
        &mut self,
        nums: usize,
        activation: Activation,
        loss: Loss,
        bias: bool,
    ) -> &mut Self {
        //self.network.output.set_number(n);
        //self.network.output.cells = (0..n)
        //    .map(|_| OutputCell::new(activation))
        //    .collect();
        self
    }

    /// Set output layer activation.
    pub fn set_output_layer_activation(&mut self, activation: Activation) -> &mut Self {
        //self.network.output.cells.iter_mut().for_each(|n| n.set_activation(activation));
        self
    }

    /// Set output layer loss.
    pub fn set_output_layer_loss(&mut self, loss: Loss) -> &mut Self {
        //self.network.output.cells.iter_mut().for_each(|n| n.set_loss(loss));
        self
    }

    /// Set output layer bias.
    pub fn set_output_layer_bias(&mut self, bias: bool) -> &mut Self {
        //self.network.output.cells.iter_mut().for_each(|n| n.set_bias(bias));
        self
    }
}
