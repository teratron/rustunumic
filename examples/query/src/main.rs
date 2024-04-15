#![allow(dead_code, unused_variables)]

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
}
