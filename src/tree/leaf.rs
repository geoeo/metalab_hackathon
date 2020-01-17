struct Node<T> {
    pub operator: fn(left:T, right:T) -> T,
    pub left:Option<Box<Node<T>>>,
    pub right:Option<Box<Node<T>>>
}

impl<T> Node<T> {

    fn insert () -> () {}

    fn insert_left (self, val : Node<T>) -> Self {
        let new_node = Node::new(val);
        self
    }

    fn insert_right () -> () {}

    fn find () -> () {}

    fn operation () -> () {}

}







