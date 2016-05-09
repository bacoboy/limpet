extern crate rustc_serialize;
extern crate docopt;
use docopt::Docopt;

use std::env;
use std::process::exit;

extern crate liquid;
use liquid::Context;
use liquid::Value;
// use liquid::{Renderable, LiquidOptions, Context, Value};

// extern crate walkdir;
// use walkdir::WalkDir;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = "
Usage: limpet [options] <source> <dest>
       limpet [options] --recursive <dir>
       limpet (--help | --version)

Options:
    -h, --help              Show this message.
    -v, --version           Show the version of limpet.
    -r, --recursive         Scan for files with extension from given directory.  Outputs to same directory.
    -e, --extension=EXT     Override the default extention for recursive scans [default: liquid].
";

#[derive(RustcDecodable)]
struct Args {
    arg_source: String,
    arg_dest: String,
    flag_extension: String,
    flag_recursive: bool,
    arg_dir: Option<String>,
    flag_help: bool,
    flag_version: bool
}

// fn print_usage(program: &str, opts: Options) {
//     let brief = format!(r#"Usage:
//   {} SRC_FILE DEST_FILE
// "#;
//     let brief = format!("Usage: {} SRC_FILE DEST_FILE [options]", program);
//     print!("{}", opts.usage(&brief));
// }

fn main() {
    let args:Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    // Dump arguments - DEBUG
    // println!("{:?}", args);

    // Build a data context for the liquid templates
    let mut data = Context::new();
    for (key, value) in env::vars() {
        // println!("key: {} value: {}", &key, &value);
        data.set_val(&key, Value::Str(value));
    }

    if args.flag_version {
        println!("liquid v{}", VERSION);
        exit(0);
    }

    if args.flag_recursive {
    } else {
    }

    println!("Hello, world!");
}
