#[derive(Copy, Clone,PartialEq,Debug)]
struct Leaf<T> {
    pub val: T,
}


struct Node<T> {
    pub operator: fn(left:T, right:T) -> T,
    pub left:Option<Box<Node<T>>>,
    pub right:Option<Box<Node<T>>>,
}

impl<T> Node<T> {

    fn insert () -> () {}

    fn insert_left () -> () {}

    fn insert_right () -> () {}

    fn find () -> () {}

    fn operation () -> () {}

}






