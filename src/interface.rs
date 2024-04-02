//! # Interface
//!
//!

//use super::{Float, Rustunumic};

//const MAX_ITERATION: usize = 1_000_000;

pub trait Interface<T> {
    /// Verify
    fn verify(&mut self, input: &[T], target: &[T]) -> T;

    /// Query
    fn query(&mut self, input: &[T]) -> Vec<T>;

    /// Train
    fn train(&mut self, input: &[T], target: &[T]) -> (usize, T);
}

/*impl<T: Float> Interface<T> for Rustunumic<T> {
    fn verify(&mut self, input: &[T], target: &[T]) -> T {
        //let numbers = [1, 2, 3, 4, 5, 6, 7, 8];
        //let slice = &numbers[1..5];
        //Self::new()
        todo!()
    }

    fn query(&mut self, input: &[T]) -> Vec<T> {
        let output: Vec<T> = vec![T::ZERO; 10];
        output
    }

    /// Training dataset.
    fn train(&mut self, input: &[T], target: &[T]) -> (usize, T) {
        // TODO: Result<(usize, T), Box<dyn Error>>
        if !self.is_init {
            if not self.__init(len(data_input), len(data_target)) {
                raise ValueError(f"{__name__}: not initialized")
            }
        }

        //self._input = input;
        //self._target = target;

        let mut count: usize = 1;
        let mut min_count: usize = 0;
        let mut loss: T = T::ZERO;
        let mut min_loss: T = T::ONE;

        //for count in 1..=MAX_ITERATION {
        while count <= MAX_ITERATION {
            self.calculate_values();
            loss = self.calculate_loss();

            if loss < min_loss {
                min_count = count;
                min_loss = loss;
                //self.__weights = deepcopy(self.weights)

                if loss < T::LOSS_LIMIT {
                    //self.weights = deepcopy(self.__weights);
                    //return (count, loss);
                    break;
                }
            }
            self.calculate_misses().calculate_weights();
            count += 1;
        }

        if min_count > 0 {
            //self.weights = deepcopy(self.__weights);
        }

        (count, loss)
    }
}*/
