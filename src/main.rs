use std::str::FromStr;

use clap::{App, Arg};

use lib::{print_bw_server, print_server_usage_bw};

mod lib;

fn main() {
    let app = App::new("Streaming_calc_rust")
        .version("0.1.0")
        .author("sycured")
        .subcommand(
            App::new("bwserver")
                .about("Determine necessary server bandwidth")
                .arg(
                    Arg::with_name("nblisteners")
                        .help("number of listeners")
                        .empty_values(false),
                )
                .arg(
                    Arg::with_name("bitrate")
                        .help("bitrate in kb/s")
                        .empty_values(false),
                ),
        )
        .subcommand(
            App::new("usagebw")
                .about("Determine the amount of data used for the streaming")
                .arg(
                    Arg::with_name("nblisteners")
                        .help("number of listeners")
                        .empty_values(false),
                )
                .arg(
                    Arg::with_name("bitrate")
                        .help("bitrate in kb/s")
                        .empty_values(false),
                )
                .arg(
                    Arg::with_name("nbdays")
                        .help("number of days")
                        .empty_values(false),
                )
                .arg(
                    Arg::with_name("nbhours")
                        .help("number of hours by days")
                        .empty_values(false),
                ),
        )
        .get_matches();

    match app.subcommand() {
        ("bwserver", Some(bwserver_args)) => print_bw_server(
            f32::from_str(bwserver_args.value_of("nblisteners").unwrap()).unwrap(),
            f32::from_str(bwserver_args.value_of("bitrate").unwrap()).unwrap(),
        ),
        ("usagebw", Some(usagebw_args)) => print_server_usage_bw(
            f32::from_str(usagebw_args.value_of("nblisteners").unwrap()).unwrap(),
            f32::from_str(usagebw_args.value_of("bitrate").unwrap()).unwrap(),
            f32::from_str(usagebw_args.value_of("nbdays").unwrap()).unwrap(),
            f32::from_str(usagebw_args.value_of("nbhours").unwrap()).unwrap(),
        ),
        _ => println!("{}", app.usage()),
    }
}
