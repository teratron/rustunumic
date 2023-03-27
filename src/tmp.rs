struct Person<T> {
    device: T,
}

impl<T: Sender + Printer> Person<T> {
    fn send_message(&self, message: &str) {
        self.device.print(message);
        self.device.send(message);
    }
}

trait Printer {
    fn print(&self, message: &str);
}

trait Sender {
    fn send(&self, message: &str);
}

struct Smartphone {}

impl Printer for Smartphone {
    fn print(&self, message: &str) {
        println!("{}", message);
    }
}

impl Sender for Smartphone {
    fn send(&self, message: &str) {
        println!("Сообщение {} отправлено", message);
    }
}

fn main() {
    let iphone = Smartphone {};
    let tom = Person { device: iphone };
    tom.send_message("Hello Rust!");
}
