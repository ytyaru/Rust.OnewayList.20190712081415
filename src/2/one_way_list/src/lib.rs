#[derive(Debug, PartialEq)]
struct Node {
    item: i32,
    next: Option<Box<Node>>,
}
impl Node {
    fn new(item: i32) -> Self { Self { item: item, next: None } }
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


    /*
    #[test]
    fn struct_Node() {
        let first = Node { item: 0, next: None };
        let second = Node { item: 1, next: Some(Box::new(first)) };
        assert_eq!(first.item, 0);
//        assert_eq!(first.next, second);
        assert_eq!(second.item, 1);
//        assert_eq!(second.next, None);
    }
    */
}
