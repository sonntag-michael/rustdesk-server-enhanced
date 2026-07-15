// https://tools.ietf.org/rfc/rfc5128.txt
// https://blog.csdn.net/bytxl/article/details/44344855

use clap::Arg;
use flexi_logger::*;
use hbb_common::{bail, config::RENDEZVOUS_PORT, ResultType};
use hbbs::{common::*, *};

const RMEM: usize = 0;

fn main() -> ResultType<()> {
    let _logger = Logger::try_with_env_or_str("info")?
        .log_to_stdout()
        .format(opt_format)
        .write_mode(WriteMode::Async)
        .start()?;
    let args = vec![
        Arg::new("configfile").short('c').long("config").value_parser(clap::builder::NonEmptyStringValueParser::new()).help("Sets a custom config file"),
        Arg::new("port").short('p').long("port").value_parser(clap::builder::NonEmptyStringValueParser::new()).default_value(RENDEZVOUS_PORT.to_string()).help("Sets the listening port"),
        Arg::new("serial").short('s').long("serial").value_parser(clap::value_parser!(u16)).default_value("0".to_string()).help("Sets configure update serial number"),
        Arg::new("rendezvous-servers").short('R').long("rendezvous-servers").value_parser(clap::builder::NonEmptyStringValueParser::new()).help("Sets rendezvous servers, separated by comma"),
        Arg::new("software URL").short('u').long("software-url").value_parser(clap::builder::NonEmptyStringValueParser::new()).help("Sets a custom config file"),
        Arg::new("relay-servers").short('r').long("relay-servers").value_parser(clap::builder::NonEmptyStringValueParser::new()).help("Sets the default relay servers, separated by comma"),
        Arg::new("UDP-buffer-size").short('M').long("rmem").value_parser(clap::builder::NonEmptyStringValueParser::new()).default_value(RMEM.to_string()).help("Sets UDP recv buffer size, set system rmem_max first, e.g., sudo sysctl -w net.core.rmem_max=52428800. vi /etc/sysctl.conf, net.core.rmem_max=52428800, sudo sysctl –p"),
        Arg::new("mask").long("mask").value_parser(clap::builder::NonEmptyStringValueParser::new()).help("Determine if the connection comes from LAN, e.g. 192.168.0.0/16"),
        Arg::new("key").short('k').long("key").value_parser(clap::builder::NonEmptyStringValueParser::new()).help("Only allow the client with the same key"),
    ];
    init_args(&args, "hbbs", "RustDesk ID/Rendezvous Server");
    let port = get_arg_or("port", RENDEZVOUS_PORT.to_string()).parse::<i32>()?;
    if port < 3 {
        bail!("Invalid port");
    }
    let rmem = get_arg("rmem").parse::<usize>().unwrap_or(RMEM);
    let serial: i32 = get_arg("serial").parse().unwrap_or(0);
    crate::common::check_software_update();
    RendezvousServer::start(port, serial, &get_arg_or("key", "-".to_owned()), rmem)?;
    Ok(())
}
