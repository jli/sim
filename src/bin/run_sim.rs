use log;
use pretty_env_logger;

use sim;

fn main() {
    // note: using log::set_max_level didn't work. not sure why.
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    println!("log level: {}", log::max_level());
    log::info!("loading world...");
    let world = sim::World::new();
    log::info!("{}", world.stats());
    log::info!("...halting world.");
}
