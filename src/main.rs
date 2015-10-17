extern crate libc;
extern crate getopts;
use getopts::Options;
use std::env;
use std::str::FromStr;

#[macro_use]
extern crate log;
extern crate env_logger; // TODO: replace

extern crate pcap;
use pcap::*;

extern crate byteorder;

#[macro_use]
extern crate enum_primitive;
extern crate num;

#[allow(dead_code)]
mod nl;

#[derive(Debug)]
#[derive(Default)]
struct Args {
    input: Option<String>,
    netlink_family: Option<nl::netlink::NetlinkFamily>,
}

fn parse_options() -> Args {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut a = Args::default();

    let mut opts = Options::new();
    opts.optopt("i", "input", "input file name", "NAME");
    opts.optopt("", "netlink_family", "filter for one netlink_family (\
                NETLINK_ROUTE, NETLINK_GENERIC, etc)", "FAMILY");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }
    a.input = matches.opt_str("i");
    if a.input.is_none() {
        println!("ERROR: we need an input file.");
        print_usage(&program, opts);
        std::process::exit(0);
    }
    let netlink_family = matches.opt_str("netlink_family");
    a.netlink_family = match netlink_family {
        // This is confusing. &*s = explicitly reborrowing String as &str.
        Some(s) => Some(nl::netlink::NetlinkFamily::from_str(&*s).unwrap()),
        None => None,
    };
    a
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_packets(args: &Args) {
    // open a new capture from the test.pcap file we wrote to above
    // let path = args.input.unwrap();
	let mut cap = match args.input {
        Some(ref s) => Capture::from_file(s).unwrap(),
        None => panic!(),
    };

    let mut p: i32 = 0;
    while let Some(packet) = cap.next() {
        p = p + 1;
        let nlmsg = nl::NlMsg::read(packet.data);

        match args.netlink_family {
            Some(ref f) => if *f != nlmsg.netlink_family { continue },
            None => (),
        };

        println!("packet[{}] = {}", p, nlmsg);
    }
}

fn main() {
    env_logger::init().unwrap();
    let args: Args = parse_options();
    debug!("args = {:?}", args);

    print_packets(&args);
}
