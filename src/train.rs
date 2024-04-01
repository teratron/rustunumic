use super::Float;

fn train<T: Float>(_input: Vec<T>, _target: Vec<T>) -> (usize, T) {
    let count: usize = 0;
    let loss: T = T::ZERO;
    (count, loss)
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
