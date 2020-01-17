#[derive(Clone)]
struct Node<T> where T: Clone{
    pub val: Option<T>,
    pub operator: fn(left:T, right:T) -> T,
    pub left:Option<Box<Node<T>>>,
    pub right:Option<Box<Node<T>>>,
}


impl<T> Node<T> where T:Clone{

    fn insert () -> () {}

    fn insert_left () -> () {}

    fn insert_right () -> () {}

    fn find () -> () {}

    fn operation () -> () {}

}






