#[derive(Debug, PartialEq)]
struct Node {
    item: i32,
    next: Option<Box<Node>>,
}
impl Node {
    fn new(item: i32) -> Self { Self { item: item, next: None } }
    fn push(&mut self, item: i32) {
        let new_node = Node::new(item);
        self.next = Some(Box::new(new_node));
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn Node_new() {
        let first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);
        let second = Node::new(1);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);
    }
    #[test]
    fn Node_new_2() {
        let mut first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);
        let mut second = Node::new(1);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);
        first.next = Some(Box::new(first));
    }
    #[test]
    fn Node_new_3() {
        let mut first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);
        let mut second = Node::new(1);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);
        first.next = Some(Box::new(first));
        let mut third = Node::new(2);
        assert_eq!(third.item, 2);
        assert_eq!(third.next, None);
        second.next = Some(Box::new(third));
    }
    #[test]
    fn Node_push() {
        let mut first = Node::new(0);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, None);

        first.push(1);
        assert_eq!(first.item, 0);
        assert_eq!(first.next, Some(Box::new(Node { item: 1, next: None })));
        let mut second = first.next.unwrap();
        assert_eq!(second.item, 1);
        assert_eq!(second.next, None);

        second.push(2);
        assert_eq!(second.item, 1);
        assert_eq!(second.next, Some(Box::new(Node { item: 2, next: None })));
        let mut third = second.next.unwrap();
        assert_eq!(third.item, 2);
        assert_eq!(third.next, None);
    }
}
