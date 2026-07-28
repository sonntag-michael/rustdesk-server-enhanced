use clap::{Command, Arg};
mod common;
mod relay_server;
use flexi_logger::*;
use hbb_common::{config::RELAY_PORT, ResultType};
use relay_server::*;
mod version;

fn main() -> ResultType<()> {
    let _logger = Logger::try_with_env_or_str("info")?
        .log_to_stdout()
        .format(opt_format)
        .write_mode(WriteMode::Async)
        .start()?;
    let args = vec![
        Arg::new("bind").short('b').long("bind").value_parser(clap::builder::NonEmptyStringValueParser::new()).help("Sets the IP address to bind to (default: all interfaces)"),
        Arg::new("port").short('p').long("port").value_parser(clap::builder::NonEmptyStringValueParser::new()).default_value(RELAY_PORT.to_string()).help("Sets the listening port"),
        Arg::new("key").short('k').long("key").value_parser(clap::builder::NonEmptyStringValueParser::new()).help("Only allow the client with the same key"),
    ];
    let matches = Command::new("hbbr")
        .version(version::VERSION)
        .author("Purslane Ltd. <info@rustdesk.com>")
        .about("RustDesk Relay Server")
        .args(args)
        .get_matches();
    if let Ok(v) = ini::Ini::load_from_file(".env") {
        if let Some(section) = v.section(None::<String>) {
            section.iter().for_each(|(k, v)| common::set_arg(k, v));
        }
    }
    let mut port = RELAY_PORT;
    if let Some(v) = common::get_arg_opt("PORT") {
        let v: i32 = v.parse().unwrap_or_default();
        if v > 0 {
            port = v + 1;
        }
    }
    let bind_default = common::get_arg("BIND");
    let bind = matches
        .get_one::<String>("bind")
        .unwrap_or(&bind_default);
    let bind_addr = common::parse_bind_address(&bind)?;
    let key_default = common::get_arg("KEY");
    let key = matches
        .get_one::<String>("key")
        .unwrap_or(&key_default);
    start_with_bind(
        bind_addr,
        matches.get_one::<String>("port").unwrap_or(&port.to_string()),
        &key,
    )?;
    Ok(())
}
