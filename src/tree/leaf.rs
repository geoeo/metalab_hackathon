#[derive(Clone)]
struct Node<T> where T: Clone{
    pub val: T,
    pub operator: fn(left:T, right:T) -> T,
<<<<<<< HEAD
    pub left:Option<Box<Node<T>>>,
    pub right:Option<Box<Node<T>>>
=======
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
>>>>>>> 4d68439c9ab13c11a415dce2942b02e6b1e1e867
}

impl<T> Node<T> where T:Clone {

    pub fn insert_left (&mut self, node: Node<T>) -> &mut Self {
        self.left = Some(Box::new(node));
        self
    }

    pub fn insert_right (&mut self, node: Node<T>) -> &mut Self {
        self.right = Some(Box::new(node));
        self
    }

<<<<<<< HEAD
    fn insert_left (self, val : Node<T>) -> Self {
        let new_node = Node::new(val);
        self
    }
=======
    pub fn eval(mut self)->T {
        match (self.left, self.right) {
            (Some(left_box), Some(right_box)) => (self.operator)(left_box.eval(),right_box.eval()),
            (_,_) => self.val
>>>>>>> 4d68439c9ab13c11a415dce2942b02e6b1e1e867

        }
    }

    pub fn find () -> () {}
}






