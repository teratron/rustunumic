pub trait Interface<T> {
    /// verify
    fn verify(&self, input: Vec<T>, target: Vec<T>) -> T;

    /// query
    fn query(&self, input: Vec<T>) -> Vec<T>;

    /// train
    fn train(&self, _input: Vec<T>, _target: Vec<T>) -> (usize, T);
}

/* impl Interface<f64> for Rustunumic<f64> {
    //type Float = f64;

    fn verify(&self, _input: Vec<f64>, _target: Vec<f64>) -> f64 {
        0.
    }
    fn query(&self, _input: Vec<f64>) -> Vec<f64> {
        vec![1., 2., 3.]
    }
    fn train(&self, _input: Vec<f64>, _target: Vec<f64>) -> (usize, f64) {
        (42, 0.5)
    }
}*/
