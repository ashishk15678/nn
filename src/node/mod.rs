struct Node {
    data: f32,
    next: Option<Box<Node>>,
    weight: f32,
    active: bool,
    activation: Option<i32>,
}

impl Node {
    pub fn new(data: f32, weight: f32) -> Node {
        Node {
            data,
            next: None,
            weight,
            active: false,
            activation: None,
        }
    }
}
