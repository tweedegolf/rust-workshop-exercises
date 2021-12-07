fn main() {
    println!("{}", multiply(10, 20));
}

fn multiply(a: i32, b: i32) {
    a * b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_multiplies() {
        assert!(multiply(10, 10) == 100);
        assert!(multiply(2, 2) == 4);
    }
}
