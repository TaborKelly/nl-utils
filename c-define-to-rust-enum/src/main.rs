extern crate getopts;
use std::env;
use getopts::Options;
use std::cmp::Ordering;
use std::io::{Write, BufRead, BufReader, BufWriter};
use std::fs::{File, OpenOptions};

#[macro_use]
extern crate log;
extern crate env_logger; // TODO: replace

extern crate regex;

#[derive(Debug)]
#[derive(Default)]
struct Args {
    input: Option<String>,
    output: Option<String>,
    name: String,
    display: bool,
    fromstr: bool,
    fromprimative: bool,
}

trait FormatOutput {
    fn write(&self, w: &mut Write, name: &String, vec: &Vec<CEnum>);
}

fn parse_options() -> Args {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut a = Args::default();

    let mut opts = Options::new();
    opts.optopt("i", "input", "input file name (stdin if not specified)", "NAME");
    opts.optopt("o", "output", "output file name (stdout if not specified)", "NAME");
    opts.optopt("", "name", "the enum name (Name if not specified)", "NAME");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("a", "all", "implment all of the traits (equivalent to \
                 --display --fromprimative --fromstr)");
    opts.optflag("", "display", "implement the std::fmt::Display trait");
    opts.optflag("", "fromprimative", "implement the num::traits::FromPrimitive trait");
    opts.optflag("", "fromstr", "implement the std::str::FromStr trait");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }
    a.input = matches.opt_str("i");
    a.output = matches.opt_str("o");
    let name = matches.opt_str("name");
    // apply default name
    a.name = name.unwrap_or(String::from("Name"));
    a.display = matches.opt_present("display");
    a.fromprimative = matches.opt_present("fromprimative");
    a.fromstr = matches.opt_present("fromstr");
    if matches.opt_present("all") {
        a.display = true;
        a.fromprimative = true;
        a.fromstr = true;
    }

    a
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} <options>\n\
                        Crudely converts C #defines into Rust enums.",
                        program);
    print!("{}", opts.usage(&brief));
}

/// Return a sorted Vec of CEnum structs
fn parse_buff<T: BufRead>(read: T) -> Vec<CEnum> {
    use std::str::FromStr;
    use regex::Regex;
    let re = Regex::new(r"^#define[:space:]+([:graph:]+)[:space:]+([:digit:]+)").unwrap();
    let mut v: Vec<CEnum> = Vec::new();

    for line in read.lines() {
        let s = line.unwrap();
        for cap in re.captures_iter(&s) {
            let i: i32 = FromStr::from_str(cap.at(2).unwrap()).unwrap();
            v.push(CEnum::new(i, cap.at(1).unwrap()));
        }
    }

    v.sort();
    v
}

fn get_input(args: &Args) -> Vec<CEnum> {
    match args.input {
        Some(ref s) => {
            let f = File::open(s).unwrap();
            let r = BufReader::new(f);
            parse_buff(r)
        }
        None => {
            let r = BufReader::new(std::io::stdin());
            parse_buff(r)
        }
    }
}

struct FormatOutputFromPrimative;
impl FormatOutput for FormatOutputFromPrimative {
    fn write(&self, w: &mut Write, name: &String, vec: &Vec<CEnum>){
        w.write(format!("impl ::num::traits::FromPrimitive for {} {{\n", name).as_bytes()).unwrap();
        w.write(format!("    #[allow(dead_code)]\n").as_bytes()).unwrap();
        w.write(format!("    fn from_i64(n: i64) -> Option<Self> {{\n").as_bytes()).unwrap();
        w.write(format!("        match n {{\n").as_bytes()).unwrap();
        for v in vec {
            w.write(format!("            {} => Some({}::{}),\n", v.i, name, v.s).as_bytes()).unwrap();
        }
        w.write(format!("            _ => None\n").as_bytes()).unwrap();
        w.write(format!("        }}\n").as_bytes()).unwrap();
        w.write(format!("    }}\n").as_bytes()).unwrap();
        w.write(format!("    #[allow(dead_code)]\n").as_bytes()).unwrap();
        w.write(format!("    fn from_u64(n: u64) -> Option<Self> {{\n").as_bytes()).unwrap();
        w.write(format!("        match n {{\n").as_bytes()).unwrap();
        for v in vec {
            w.write(format!("            {} => Some({}::{}),\n", v.i, name, v.s).as_bytes()).unwrap();
        }
        w.write(format!("            _ => None\n").as_bytes()).unwrap();
        w.write(format!("        }}\n").as_bytes()).unwrap();
        w.write(format!("    }}\n").as_bytes()).unwrap();
        w.write(format!("}}\n").as_bytes()).unwrap();
    }
}

struct FormatOutputDisplay;
impl FormatOutput for FormatOutputDisplay {
    fn write(&self, w: &mut Write, name: &String, vec: &Vec<CEnum>){
        w.write(format!("impl ::std::fmt::Display for {} {{\n", name).as_bytes()).unwrap();
        w.write(format!("    #[allow(dead_code)]\n").as_bytes()).unwrap();
        w.write(format!("    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {{\n").as_bytes()).unwrap();
        w.write(format!("        match *self {{\n").as_bytes()).unwrap();
        for v in vec {
            w.write(format!("            {}::{} => write!(f, \"{}\"),\n", name, v.s, v.s).as_bytes()).unwrap();
        }
        w.write(format!("        }}\n").as_bytes()).unwrap();
        w.write(format!("    }}\n").as_bytes()).unwrap();
        w.write(format!("}}\n").as_bytes()).unwrap();
    }
}

struct FormatOutputFromStr;
impl FormatOutput for FormatOutputFromStr {
    fn write(&self, w: &mut Write, name: &String, vec: &Vec<CEnum>){
        w.write(format!("impl ::std::str::FromStr for {} {{\n", name).as_bytes()).unwrap();
        w.write(format!("    type Err = ();\n").as_bytes()).unwrap();
        w.write(format!("    #[allow(dead_code)]\n").as_bytes()).unwrap();
        w.write(format!("    fn from_str(s: &str) -> Result<Self, Self::Err> {{\n").as_bytes()).unwrap();
        w.write(format!("        match s {{\n").as_bytes()).unwrap();
        for v in vec {
            w.write(format!("            \"{}\" => Ok({}::{}),\n", v.s, name, v.s).as_bytes()).unwrap();
        }
        w.write(format!("            _ => Err( () )\n").as_bytes()).unwrap();
        w.write(format!("        }}\n").as_bytes()).unwrap();
        w.write(format!("    }}\n").as_bytes()).unwrap();
        w.write(format!("}}\n").as_bytes()).unwrap();
    }
}

struct FormatOutputEnum;
impl FormatOutput for FormatOutputEnum {
    fn write(&self, w: &mut Write, name: &String, vec: &Vec<CEnum>){
        w.write(format!("#[allow(dead_code, non_camel_case_types)]\n").as_bytes()).unwrap();
        w.write(format!("enum {} {{\n", name).as_bytes()).unwrap();

        for v in vec {
            w.write(format!("    {} = {},\n", v.s, v.i).as_bytes()).unwrap();
        }

        w.write(format!("}}\n").as_bytes()).unwrap();
    }
}

fn write_factory(args: &Args) -> Box<Write> {
    match args.output {
        Some(ref s) => {
            let f = OpenOptions::new().write(true)
                                      .create(true)
                                      .truncate(true)
                                      .open(s).unwrap();
            let w = BufWriter::new(f);
            Box::new(w)
        }
        None => {
            let w = BufWriter::new(std::io::stdout());
            Box::new(w)
        }
    }
}

fn main() {
    use std::fs::File;
    env_logger::init().unwrap();
    let args: Args = parse_options();
    debug!("args = {:?}", args);

    let mut fov: Vec<Box<FormatOutput>> = Vec::new();
    fov.push(Box::new(FormatOutputEnum));
    if args.fromstr { fov.push(Box::new(FormatOutputFromStr)); }
    if args.display { fov.push(Box::new(FormatOutputDisplay)); }
    if args.fromprimative { fov.push(Box::new(FormatOutputFromPrimative)); }

    let vi = get_input(&args);
    let mut w = write_factory(&args);

    for vw in fov {
        vw.write(&mut w, &args.name, &vi);
    }
}

#[derive(Debug)]
struct CEnum {
    i: i32,
    s: String,
}
impl CEnum {
    fn new(i: i32, s: &str) -> CEnum {
        CEnum { i:i, s: String::from(s) }
    }
}
impl ::std::cmp::Eq for CEnum {}
impl ::std::cmp::PartialEq for CEnum {
    fn eq(&self, other: &Self) -> bool {
        if self.i == other.i {
            return true;
        }
        false
    }
}
impl ::std::cmp::PartialOrd for CEnum {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.i < other.i {
            return Some(Ordering::Less);
        }
        else if self.i > other.i {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Equal)
    }
}
impl ::std::cmp::Ord for CEnum {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.i < other.i {
            return Ordering::Less;
        }
        else if self.i > other.i {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

#[test]
fn teste_CENum_order() {
    let a = CEnum::new(0, "");
    let b = CEnum::new(1, "");
    let c = CEnum::new(2, "");
    let d = CEnum::new(0, "");
    assert!(a < b);
    assert!(b < c);
    assert!(a < c);
    assert!(b > a);
    assert!(c > b);
    assert!(c > a);
    assert!(a == d);
}

#[test]
fn test_parse_buff() {
    use std::io::Cursor;
    let s = "#define NETLINK_ROUTE 0\n\
    #define NETLINK_UNUSED 1\n\
    #define NETLINK_FIREWALL 3\n\
    #define NETLINK_SOCK_DIAG 4\n\
    #define NETLINK_GENERIC 16";

    let buff = Cursor::new(s.as_bytes());

    let v = parse_buff(buff);

    assert!(v[0].i == 0); assert!(v[0].s == "NETLINK_ROUTE");
    assert!(v[1].i == 1); assert!(v[1].s == "NETLINK_UNUSED");
    assert!(v[2].i == 3); assert!(v[2].s == "NETLINK_FIREWALL");
    assert!(v[3].i == 4); assert!(v[3].s == "NETLINK_SOCK_DIAG");
    assert!(v[4].i == 16); assert!(v[4].s == "NETLINK_GENERIC");
}
