#![allow(unused)]

use rustunumic::Rustunumic;

const JSON_STREAM: &'static str = r#"{
    "bias": true,
    "activation_mode": 3,
    "loss_mode": 0,
    "loss_limit": 1e-10,
    "rate": 0.3,
    "weights": [
        [
            [-2.5128086, 2.6974556, 3.034397, -2.4341068],
            [-1.2436904, -1.1729956, 4.4001436, -2.1053333],
            [-1.2884712, 2.5544305, 10.886107, -2.1163273],
            [3.9765725, 3.646633, 4.741202, -3.8852577],
            [8.725591, 3.0480642, 3.0672483, -7.115494]
        ],
        [
            [3.7148979, 2.9444046, -5.72786, 2.2840204, -1.6592604, 0.33781952],
            [1.8408697, 2.070344, -6.0672054, 3.9654624, -2.7668004, 2.3363395],
            [1.8098677, 2.2063692, 0.08325871, -4.959725, 5.3901534, 1.0965135]
        ],
        [
            [2.1007898, 6.552546, -5.262143, -1.1054513],
            [-6.4693666, -4.019415, -3.8858104, 6.2537074]
        ]
    ]
}"#;

fn main() {
    // Returns a new neural network instance.
    let mut rn = Rustunumic::<f32>::new(/*JSON_STREAM*/);

    // Input dataset.
    let data_input = [0.27, 0.31, 0.52];

    // Getting the results of the trained network.
    let data_output = rn.query(&data_input);
    print!("Query: {data_output:?}");
}

/*#![allow(dead_code, unused_variables)]

trait Area {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    length: f64,
}

impl Rectangle {
    fn new(width: f64, length: f64) -> Option<Self> {
        if width <= 0. || length <= 0. {
            None
        } else {
            Some(Rectangle { width, length })
        }
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.length
    }
}

struct Square {
    rectangle: Rectangle,
}

impl Square {
    fn new(side: f64) -> Option<Self> {
        Some(Square {
            rectangle: Rectangle::new(side, side)?,
        })
    }
}

impl Area for Square {
    fn area(&self) -> f64 {
        self.rectangle.area()
    }
}

fn print_type<T: 'static>() {
    match std::any::TypeId::of::<T>() {
        t if t == std::any::TypeId::of::<f32>() => println!("f32"),
        t if t == std::any::TypeId::of::<f64>() => println!("f64"),
        _ => println!("Unknown type"),
    }
}

fn identify_type<T>(value: T) {
    match value {
        // Explicitly check for f32 and f64 using `is` operator
        //val if val.is::<f32>() => println!("f32"), // is - error
        //val if val.is::<f64>() => println!("f64"),
        _ => println!("Неизвестный тип (Unknown type)"), // Handle other types with a descriptive message
    }
}

fn main() {
    print_type::<f32>();
    print_type::<f64>();
    print_type::<i32>();

    let my_float_32: f32 = 3.14;
    let my_float_64: f64 = 2.718;
    let my_string: String = "Hello, world!".to_string();

    identify_type(my_float_32);
    identify_type(my_float_64);
    identify_type(my_string);

    let rect1 = Rectangle::new(3., 5.).unwrap();
    let rect2 = Rectangle::new(4., 6.).unwrap();

    let sq1 = Square::new(8.).unwrap();
    let sq2 = Square::new(4.).unwrap();

    let figures_with_area: [&dyn Area; 4] = [&rect1, &rect2, &sq1, &sq2];
    for f in figures_with_area {
        println!("Площадь равна {}", f.area());
    }

    println!("---");

    let figures_without_area: Vec<Box<&dyn Area>> = vec![Box::new(&rect1), Box::new(&sq1)];
    for f in figures_without_area {
        println!("Площадь равна {}", f.area());
    }
}*/
