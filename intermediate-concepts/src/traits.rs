pub trait Vehicle {
    fn forward(&self) -> String;
    fn backward() -> String;

    fn turn_ignition() -> String {
        String::from("vroom vroom")
    }
}

pub struct Car {
    pub color: String,
}

impl Vehicle for Car {
    fn forward(&self) -> String {
        format!("The {} car is moving forward", self.color)
    }

    fn backward() -> String {
        String::from("Started moving backward")
    }
}
