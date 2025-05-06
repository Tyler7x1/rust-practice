enum TrafficLight {
    Red(u32),
    Yellow(u32),
    Green(u32),
}

impl TrafficLight {
    fn show_instructions(&self) {
        match self {
            TrafficLight::Red(seconds) => println!("Red light stop for {} seconds", seconds),
            TrafficLight::Yellow(seconds) => println!("Yellow light stop for {} seconds", seconds),
            TrafficLight::Green(seconds) => println!("Green light stop for {} seconds", seconds),
        }
    }
}

fn main() {
    let red = TrafficLight::Red(30);
    let yellow = TrafficLight::Yellow(15);
    let green = TrafficLight::Green(10);

    red.show_instructions();
    yellow.show_instructions();
    green.show_instructions();
    
}