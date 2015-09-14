extern crate libc;
extern crate getopts;
use getopts::Options;
use std::env;

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
}

fn parse_options() -> Args {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut a = Args::default();

    let mut opts = Options::new();
    opts.optopt("i", "input", "input file name", "NAME");
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
    a
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn print_packets(path: String) {
    // open a new capture from the test.pcap file we wrote to above
	let mut cap = Capture::from_file(path).unwrap();

    let mut p: i32 = 1;
    while let Some(packet) = cap.next() {
        println!("packet = {}", p);
        let nlmsg = nl::NlMsg::read(packet.data);
        println!("nlmsg = {:?}", nlmsg);
        println!("nlmsg = {}", nlmsg);
        p = p + 1;
    }
}

fn main() {
    env_logger::init().unwrap();
    let args: Args = parse_options();
    debug!("args = {:?}", args);

    match args.input {
        Some(x) => print_packets(x),
        _ => panic!(),
    }
}
