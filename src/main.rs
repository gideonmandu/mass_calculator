use std::io;

fn calculate_weight_on_mass(weight: &f32) -> f32 {
    ( weight / 9.81 ) * 3.711
}

fn main() {
    println!("ENter your weight in Kgs.");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let weight:f32 =input.trim().parse().unwrap();
    let mut mars_weight = calculate_weight_on_mass(&weight);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on mars {} grams", mars_weight);
}
