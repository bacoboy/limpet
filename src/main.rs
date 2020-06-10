extern crate docopt;
extern crate rustc_serialize;
extern crate liquid;
extern crate walkdir;

use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use docopt::Docopt;
use liquid::Template;
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
    // flag_help: bool,  Don't need this - it's automatically processed by docopt
    flag_version: bool
}

fn process_single_file(source: &PathBuf, destination: &PathBuf, context: &mut liquid::Object) -> Result<(), String> {
    // Read in full template as string
    match File::open(source.as_path()) {
        Ok(mut f) => {
            // Double check that the source isn't a directory
            if let Ok(metadata) = source.metadata() {
                if metadata.is_dir() {
                    return Err(format!("Source file {:?} can't be a directory.  Did you mean to use the --recursive option?", &source));
                }

                // TODO: save permissions to set file with later?
                // let permissions = metadata.permissions();

            } else {
                return Err(format!("Problem reading source file metadata <{:?}>: Could be permissions.", &source));
            }

            // Read template and process
            let mut contents: Vec<u8> = Vec::new();
            let _ = f.read_to_end(&mut contents).expect("Couldn't read whole file");
            // println!("Read {} bytes", size);
            let template_string = String::from_utf8(contents).unwrap();
            let template: Template = liquid::ParserBuilder::with_stdlib().build().unwrap().parse(&template_string).unwrap();
            if let Ok(output_string) = template.render(context) {
                // println!("rendered file is {:?}", &output_string);
                match File::create(&destination) {
                    Ok(mut output_file) => {
                        if let Err(e) = output_file.write_all(&output_string.into_bytes()) {
                            return Err(format!("Error writing file {}: {}", destination.to_str().unwrap(), &e));
                        }
                        // I don't think we need to flush/close in rust.
                        // output_file.flush();
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(format!("Destination file <{}>: {}", destination.to_str().unwrap(), &e));
                    }
                }
            } else {
                return Err(format!("Unknown error while rendering the template for {}", source.to_str().unwrap()));
            }
        }
        Err(e) => {
            return Err(format!("{:?}", e));
        }
    }
}

fn main() {
    let args:Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    // Build a data context for the liquid templates
    let mut data = liquid::Object::new();
    for (key, value) in env::vars() {
        // println!("key: {} value: {}", &key, &value);
        data.insert(key.into(), liquid::model::Value::scalar(value));
    }

    if args.flag_version {
        println!("limpet v{}", VERSION);
        exit(0);
    }

    if args.flag_recursive {
        // Do recursive scan starting at <dir>
        println!("Scanning directory {}...", &args.arg_dir);
        for entry in WalkDir::new(&args.arg_dir) {
            match entry {
                Ok(ref e) => {
                    // println!("Scanning directory {} ", e.path().display());
                    if e.file_type().is_file() {
                        let p = e.path();
                        // println!("{} is a file", p.display());
                        if let Some(ext) = p.extension() {
                            if ext.to_str() == Some(&args.flag_extension) {
                                if let Some(parent) = p.parent() {
                                    let mut dest = PathBuf::from(parent);
                                    if let Some(file_stem) = p.file_stem() {
                                        dest.push(file_stem);
                                        // println!("Processing file {} into {}", p.display(), dest.display());
                                        match process_single_file(&p.to_path_buf(), &dest, &mut data) {
                                            Ok(_) => {
                                                println!("Processed: {} => {}", p.to_path_buf().display(), dest.display());
                                            }
                                            Err(e) => {
                                                println!("ERROR processing file {}: {:?}", p.display(), e);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(ref e) => {
                    println!("ERROR processing directory entry {:?}: {:?}", entry, e);
                }
            }
        }
    } else {
        // Do single file
        let source = PathBuf::from(&args.arg_source);
        let dest = PathBuf::from(&args.arg_dest);
        match process_single_file(&source.to_path_buf(), &dest, &mut data) {
            Ok(_) => {
                println!("Processed: {} => {}", source.display(), dest.display());
                exit(0);
            }
            Err(e) => {
                println!("ERROR processing file {}: {:?}", source.display(), e);
                exit(1);
            }
        }
    }
}
