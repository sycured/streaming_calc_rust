use clap::{crate_authors, crate_version, Arg, Command};

use streaming_calc_rust::{bw_server, server_usage_bw};
fn main() {
    let app = Command::new("Streaming_calc_rust")
        .version(crate_version!())
        .author(crate_authors!())
        .arg_required_else_help(true)
        .subcommand(
            Command::new("bwserver")
                .about("Determine necessary server bandwidth")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("nblisteners")
                        .help("number of listeners")
                        .required(true),
                )
                .arg(Arg::new("bitrate").help("bitrate in kb/s").required(true)),
        )
        .subcommand(
            Command::new("usagebw")
                .about("Determine the amount of data used for the streaming")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("nblisteners")
                        .help("number of listeners")
                        .required(true),
                )
                .arg(Arg::new("bitrate").help("bitrate in kb/s").required(true))
                .arg(Arg::new("nbdays").help("number of days").required(true))
                .arg(
                    Arg::new("nbhours")
                        .help("number of hours by days")
                        .required(true),
                ),
        )
        .get_matches();

    match app.subcommand() {
        Some(("bwserver", bwserver_args)) => bw_server(
            bwserver_args.get_one("nblisteners").unwrap(),
            bwserver_args.get_one("bitrate").unwrap(),
        ),
        Some(("usagebw", usagebw_args)) => server_usage_bw(
            usagebw_args.get_one("nblisteners").unwrap(),
            usagebw_args.get_one("bitrate").unwrap(),
            usagebw_args.get_one("nbdays").unwrap(),
            usagebw_args.get_one("nbhours").unwrap(),
        ),
        _ => unreachable!(),
    }
}
