pub struct Node {
    pub data: f32,
    pub next: Option<Box<Node>>,
    pub weight: f32,
    pub active: bool,
    pub activation: Option<i32>,
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

    pub fn set_next(&mut self, next: Node) {
        self.next = Some(Box::new(next));
    }

    pub fn activate(&mut self) {
        self.active = true;
        self.activation = Some(self.calculate_activation());
    }

    pub fn deactivate(&mut self) {
        self.active = false;
        self.activation = None;
    }

    pub fn calculate_activation(&self) -> i32 {
        // Placeholder for actual activation function
        (self.data * self.weight) as i32
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn get_activation(&self) -> Option<i32> {
        self.activation
    }
}
