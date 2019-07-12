struct Node {
    item: i32,
    next: Node, // error[E0072]: recursive type `Node` has infinite size
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
