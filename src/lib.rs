pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

// pub mod hello {
//     pub fn greetings_from_rust() -> String {
//         String::from("Hello from rust")
//     }
// }

// pub fn public_function() {
//     println!("called rary's `public_function()`");
// }
//
// fn private_function() {
//     println!("called rary's `private_function()`");
// }
//
// pub fn indirect_access() {
//     print!("called rary's `indirect_access()`, that\n> ");
//
//     private_function();
// }
