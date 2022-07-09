#[derive(Debug)]
enum TrafficLight{
    Red(u32),
    Yellow(u32),
    Green(u32),
}

trait Duration {
    fn time(&self) -> &u32;
}

impl Duration for TrafficLight {
    fn time(&self) -> &u32 {
        match self {
            TrafficLight::Red(t) => t,
            TrafficLight::Yellow(t) => t,
            TrafficLight::Green(t) => t,
        }
    }
}

fn main() {
    let red_light = TrafficLight::Red(30);
    let yellow_light = TrafficLight::Yellow(40);
    let green_light = TrafficLight::Green(50);
    println!("\nTraffic Lights:");
    println!("Red light: {:?}", red_light);
    println!("Yellow light: {:?}", yellow_light);
    println!("Green light: {:?}", green_light);
    println!("\nDuration:");
    println!("Red light time: {} seconds", red_light.time());
    println!("Yellow light time: {} seconds", yellow_light.time());
    println!("Green light time: {} seconds", green_light.time());
}
