use std::{collections::HashMap, fs};

fn main() {
    let buffer = fs::read_to_string("input3.txt").unwrap();
    let result = process(&buffer);
    dbg!(result);
}

fn process(input: &str) -> String {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        todo!();
    }
}
