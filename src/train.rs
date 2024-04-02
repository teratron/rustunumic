use super::{Float, Rustunumic};

const MAX_ITERATION: usize = 1_000_000;

impl<T: Float> Rustunumic<T> {
    /// Training dataset.
    pub fn train(&mut self, input: &[T], target: &[T]) -> (usize, T) {
        // TODO: Result<(usize, T), Box<dyn Error>>
        if !self.is_init {
            /*if not self.__init(len(data_input), len(data_target)) {
                raise ValueError(f"{__name__}: not initialized")
            } */
        }

        //self._input = input;
        //self._target = target;

        let mut count: usize = 1;
        let mut min_count: usize = 0;
        let mut loss: T = T::ZERO;
        let mut min_loss: T = T::ONE;

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
}

/*
min_loss = 1.0
min_count = 0

for count in range(1, self.MAX_ITERATION):
    if not self.__is_query:
        # if not self._params.is_query:
        self._calculate_neurons()
    else:
        # self._params.is_query = False
        self.__is_query = False

    loss = self._calculate_loss()

    self.data.count.append(count)
    self.data.loss.append(loss)

    if loss < min_loss:
        min_loss = loss
        min_count = count
        self.__weights = deepcopy(self._weights)
        # print(f"--------- {count}, {loss:.33f}, {loss.as_integer_ratio()}")  #
        if loss < self._loss_limit:
            self._weights = deepcopy(self.__weights)
            return min_count, min_loss

    # if count % 10000 == 0:
    #     # print(f"+++ {count}, {loss:.33f}, {str(loss)[str(loss).rfind('e-') + 2:]}, {(loss - prev_loss):.33f}")
    #     print(f"+++ {count}, {loss:.33f}, {loss - prev_loss}")
    # prev_loss = loss

    # ratio = loss.as_integer_ratio()[0]
    # if prev_ratio == ratio and count % 10000 == 0:
    #     print(f"******** {prev_ratio} - {ratio}")
    # prev_ratio = ratio

    self._calculate_miss()
    self._update_weights()

if min_count > 0:
    self._weights = deepcopy(self.__weights)

return min_count, min_loss
*/
