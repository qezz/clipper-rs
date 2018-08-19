extern crate libc;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
use clap::{Arg, App};

extern crate tokio;
extern crate tokio_io;

extern crate futures;

extern crate tokio_process;

extern crate clipper;

fn main() {
    unsafe {
        libc::umask(0o077);
    }

    let matches = App::new("Clipper")
        .version("0.1.0")
        .author("Sergey @qezz")
        .about("Clipboard extension")
        .arg(Arg::with_name("port")
             .short("p")
             .long("port")
             .value_name("PORT")
             .help("port to listen on")
             .takes_value(true)
             .default_value("8377"))
        .arg(Arg::with_name("address") // defaults to IPv4/IPv6 loopback.
             .short("a")
             .long("address")
             .value_name("ADDR")
             .help("address to bind to (default loopback interface)")
             .takes_value(true)
             .default_value("127.0.0.1"))
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("CFG")
             .help("path to (JSON) config file")
             .takes_value(true)
             .default_value("~/.clipper-rs.json")) // OS specific
        .arg(Arg::with_name("executable")
             .short("e")
             .long("executable")
             .value_name("EXE")
             .help("program called to write to clipboard")
             .takes_value(true)
             .default_value("pbcopy")) // OS specific
        .arg(Arg::with_name("flags")
             .short("f")
             .long("flags")
             .value_name("FLAGS")
             .help("arguments passed to clipboard executable")
             .takes_value(true)
             .default_value("") // OS specific
             .hide_default_value(true))
        // TODO: Not supported now, see env_logger
        // .arg(Arg::with_name("logfile")
        //      .short("l")
        //      .long("logfile")
        //      .value_name("LOG")
        //      .help("path to logfile")
        //      .takes_value(true)
        //      .default_value("~/Library/Logs/com.wincent.clipper.log""))
        .get_matches();

    env_logger::init();

    info!("Starting clipper");
    let cfg = clipper::Config::from_args(&matches);
    let future = clipper::Clipper::run(cfg);

    tokio::run(future);
}
