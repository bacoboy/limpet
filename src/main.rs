extern crate rustc_serialize;
extern crate docopt;
use docopt::Docopt;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

extern crate liquid;
use liquid::Context;
use liquid::Renderable;
use liquid::Template;
use liquid::Value;

// use liquid::{Renderable, LiquidOptions, Context, Value};

extern crate walkdir;
use walkdir::WalkDir;

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
    arg_dir: String,
    flag_help: bool,
    flag_version: bool
}

fn read_whole_file(mut f: &File) -> String {
    let mut contents: Vec<u8> = Vec::new();
    let size = f.read_to_end(&mut contents).expect("Couldn't read whole file");
    println!("Read {} bytes", size);
    return String::from_utf8(contents).unwrap();
}

fn main() {
    let args:Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    // Build a data context for the liquid templates
    let mut data = Context::new();
    for (key, value) in env::vars() {
        // println!("key: {} value: {}", &key, &value);
        data.set_val(&key, Value::Str(value));
    }

    if args.flag_version {
        println!("limpet v{}", VERSION);
        exit(0);
    }

    if args.flag_recursive {
        // Do recursive scan starting at <dir>
        for entry in WalkDir::new(&args.arg_dir) {
            match entry {
                Ok(ref e) => {
                    // println!("scanning {} ", e.path().display());
                    if e.file_type().is_file() {
                        let p = e.path();
                        // println!("{} is a file", p.display());
                        if let Some(ext) = p.extension() {
                            if ext.to_str() == Some(&args.flag_extension) {
                                if let Some(parent) = p.parent() {
                                    let mut path = PathBuf::from(parent);
                                    if let Some(file_stem) = p.file_stem() {
                                        path.push(file_stem);
                                        let dest = path.as_path();
                                        println!("Processing file {} into {}", p.display(), dest.display());
                                    }
                                }
                            }
                        }
                    }
                }
                Err(_) => {
                }
            }
            // let entry = entry.unwrap();
            // println!("{}", entry.path().display());
        }
    } else {
        // Do single file
        match File::open(&args.arg_source) {
            Ok(source) => {
                if let Ok(metadata) = source.metadata() {
                    if metadata.is_dir() {
                        println!("Source file {} can't be a directory.  Did you mean to use the --recursive option?", &args.arg_source);
                        exit(1);
                    }

                    // TODO: save permissions to set file with later?
                    // let permissions = metadata.permissions();

                } else {
                    println!("Problem reading source file metadata <{}>: Could be permissions.", &args.arg_source);
                    exit(1);
                }

                // Read in full template as string
                let template_string = read_whole_file(&source);
                let template: Template = liquid::parse(&template_string, Default::default()).unwrap();
                if let Ok(Some(output_string)) = template.render(&mut data) {
                    // println!("rendered file is {:?}", &output_string);
                    match File::create(&args.arg_dest) {
                        Ok(mut destination) => {
                            if let Err(e) = destination.write_all(&output_string.into_bytes()) {
                                println!("Error writing file {}", &args.arg_dest);
                                exit(1);
                            }
                            destination.flush();
                        }
                        Err(e) => {
                            println!("Destination file <{}>: {}", &args.arg_dest, &e);
                            exit(1);
                        }
                    }
                } else {
                    println!("Error rendering the template");
                    exit(1);
                }
            }
            Err(e) => {
                println!("Source file <{}>: {}", &args.arg_source, &e);
                exit(1);
            }
        }
    }
}
