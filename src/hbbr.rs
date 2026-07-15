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
            unsafe {
                section.iter().for_each(|(k, v)| std::env::set_var(k, v));
            }
        }
    }
    let mut port = RELAY_PORT;
    if let Ok(v) = std::env::var("PORT") {
        let v: i32 = v.parse().unwrap_or_default();
        if v > 0 {
            port = v + 1;
        }
    }
    start(
        matches.get_one::<String>("port").unwrap_or(&port.to_string()),
        matches
            .get_one::<String>("key")
            .unwrap_or(&std::env::var("KEY").unwrap_or_default()),
    )?;
    Ok(())
}
