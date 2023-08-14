use std::os::macos::raw::stat;
use clap::{Args, Parser, Subcommand, ValueEnum};
use ice;

/// A cli for the ice wifi
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "ice")]
#[command(about = "hehe train fast brrr", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Display the train's status
    #[command()]
    Status,
    /// Display your trip
    #[command()]
    Trip {}

}

#[tokio::main]
async fn main() {
    let args=  Cli::parse();

    match args.command {
        Commands::Status  => {
            let status = ice::get_status().await.unwrap();
            println!("{} {} Class",  &status.train_type, &status.wagon_class);
            println!("Moving at: {} Km/h !!!", &status.speed);
            println!("Position: {}",  &status.internet);
            println!("Connectivity: {} For {} seconds",  &status.connectivity.current_state, &status.connectivity.remaining_time_seconds);
            println!("Connectivity soon: {}",  &status.connectivity.next_state);
        }
        Commands::Trip {}  => {
            let response = ice::get_trip().await.unwrap();
            let trip = response.trip;
            println!("{} To {}", trip.train_type, trip.stop_info.final_station_name);
            // TODO stop info has id's the names are inside of the stops array use that to print a pretty table

        }
    }

}


