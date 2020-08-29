#[macro_use]
extern crate clap;

use clap::{crate_authors, crate_version, App, AppSettings, Arg};

use lib::{print_bw_server, print_server_usage_bw};

mod lib;

fn main() {
    let app = App::new("Streaming_calc_rust")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("bwserver")
                .about("Determine necessary server bandwidth")
                .setting(AppSettings::ArgRequiredElseHelp)
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
                .setting(AppSettings::ArgRequiredElseHelp)
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
            value_t!(bwserver_args, "nblisteners", f32).unwrap(),
            value_t!(bwserver_args, "bitrate", f32).unwrap(),
        ),
        ("usagebw", Some(usagebw_args)) => print_server_usage_bw(
            value_t!(usagebw_args, "nblisteners", f32).unwrap(),
            value_t!(usagebw_args, "bitrate", f32).unwrap(),
            value_t!(usagebw_args, "nbdays", f32).unwrap(),
            value_t!(usagebw_args, "nbhours", f32).unwrap(),
        ),
        _ => println!("{}", app.usage()),
    }
}
