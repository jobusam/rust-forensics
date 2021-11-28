mod traffic_stats;

use clap::{App, Arg, ArgMatches};
use traffic_stats::{datapass_webstats, network_connections};

const SUB_CMD_TELEKOM_WEB_STATS: &str = "telekom-web-stats";
const SUB_CMD_NETWORK: &str = "network";
const SUB_CMD_CONNECTION: &str = "connections";
const FLAG_CONNECTION_PROCESS_INFO: &str = "process-info";

fn main() {
    let app_m = App::new("forensics")
        .version("0.0.3")
        .author("J. B.")
        .about("A collection of some analysis features to gather information about the system and the network")
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .takes_value(true)
                .help_heading(Option::Some("File to save the result")),
        )
        .subcommand(App::new(SUB_CMD_TELEKOM_WEB_STATS).about("Extract some Web Stats of Telekom LTE contract"))
        .subcommand(App::new(SUB_CMD_NETWORK).about("Statistics about the network")
            .subcommand(App::new(SUB_CMD_CONNECTION).about("Show current connections")
                .arg(Arg::new(FLAG_CONNECTION_PROCESS_INFO).short('p').long("process_info").required(false)
                    .about("Add information about the process that owns the connection")))
        )
        .get_matches();

    //let stats_file = matches.value_of("file")..unwrap_or("stats.csv");
    //println!("Use file {} to save the stats", stats_file);

    match app_m.subcommand() {
        Some((SUB_CMD_TELEKOM_WEB_STATS, _)) => datapass_webstats::check_data_limit(),
        Some((SUB_CMD_NETWORK, sub_m)) => subcommand_network(sub_m),
        _ => {} // Either no subcommand or one not tested for...
    }
}

fn subcommand_network(sub_m: &ArgMatches) {
    println!("Network statistics:");
    match sub_m.subcommand() {
        Some((SUB_CMD_CONNECTION, sub_m)) => {
            network_connections::show_connections(sub_m.is_present(FLAG_CONNECTION_PROCESS_INFO))
        }
        _ => {}
    }
}
