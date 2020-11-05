use sim;

fn main() {
    println!("...loading world...");
    let world = sim::World::new();
    println!("{}", world.stats());
}
