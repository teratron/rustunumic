use crate::Float;
use crate::{Activation, Loss, Rustunumic};

impl<T: Float> Rustunumic<'_, T> {
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
    pub fn set_hidden_layers(&mut self, hidden_layers: Vec<usize>) -> &mut Self {
        println!("hidden_layers: {hidden_layers:?}");
        let n = hidden_layers.iter().sum::<usize>();
        self.network.hidden.set_number(n);
        //self.network.cells = vec![Box::new(HiddenCell::new(Activation::Linear)); n];

        //.resize(n, Box::new(HiddenCell::new(Activation::Linear)));
        // TODO: ?
        self
    }
}
