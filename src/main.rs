extern crate tokio;

use std::error::Error;
use openrgb::OpenRGB;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Controller ID
    #[clap(short, long)]
    controller: u32,

    /// Zone ID (-1 for all zones)
    #[clap(short, long, default_value_t = -1)]
    zone: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // connect to local server
    let client = OpenRGB::connect_to(("localhost", 6742)).await?;

    // set client name
    client.set_name("image-to-openrgb").await?;

    // print protocol version
    println!(
        "connected using protocol version {}",
        client.get_protocol_version()
    );
    
    let controller_id = args.controller;
    let controller = client.get_controller(controller_id).await?;
    println!("Using controller {}: {}", controller_id, controller.name);

    let mut colors = Vec::<openrgb::data::Color>::new();
    let leds_length = if args.zone == -1 { controller.leds.len() } else { controller.zones[args.zone as usize].leds_count as usize };

    for i in 0..leds_length {
        let step = (i as f32 / leds_length as f32) * 255.0;
        colors.push(openrgb::data::Color::new(step as u8, 255 - step as u8, 0));
    }

    if args.zone != -1 {
        client.update_zone_leds(controller_id, args.zone as u32, colors).await?;
    } else {
        client.update_leds(controller_id, colors).await?;
    }

    Ok(())
}
