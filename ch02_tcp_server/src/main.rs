fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use mylib::add;

    #[test]
    pub fn case01() {
        assert_eq!(2, add(1, 1))
    }
}
