#[derive(Clone)]
struct Node<T> where T: Clone{
    pub val: T,
    pub operator: fn(left:T, right:T) -> T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
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

    pub fn eval(mut self)->T {
        match (self.left, self.right) {
            (Some(left_box), Some(right_box)) => (self.operator)(left_box.eval(),right_box.eval()),
            (_,_) => self.val

        }
    }

    pub fn find () -> () {}
}






