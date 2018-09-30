// imports {{{
extern crate failure;
// extern crate glob;
#[macro_use]
extern crate envconfig_derive;
extern crate duct;
extern crate envconfig;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate csv;
// use chrono::prelude::*;
use envconfig::Envconfig;
use failure::Error;
// use glob::glob;
use std::fs;
use std::process;
use structopt::StructOpt;
// use std::io::{self};
// use structopt::StructOpt;
// macro to create vector of strings
// macro_rules! vec_of_strings {
//     ($($x:expr),*) => (vec![$($x.to_string()),*]);
// }
// }}}
// structs and enums {{{
#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "HOME")]
    home: String,
    #[envconfig(from = "EDITOR")]
    editor: String,
}
#[derive(Debug, Deserialize)]
struct Csv {
    alias: String,
    dir: String,
}
// index 0 parsing
#[derive(Debug, StructOpt)]
struct Opt {
    file: Option<String>,
    #[structopt(subcommand)]
    cmd: Option<Command>,
}
// subcommand parsing
#[derive(Debug, StructOpt)]
enum Command {
    Push { file: Option<String> },
    Mvq { ips: String },
    Rex { test: String },
}
// initialize error

// }}}
// main {{{
fn main() {
    // Setup data
    let opt = Opt::from_args();
    let mut csv = vec![];
    let mut files = vec![];
    // Initialize config from environment variables
    let config = Config::init().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    // read csv
    if let Err(err) = read(&config.home, &mut csv) {
        println!("Error reading CSV: {}", err);
        process::exit(1);
    }
    // read paths
    if let Err(err) = paths(&config.home, &mut files) {
        println!("Error getting paths {}", err);
        process::exit(1);
    }
    // filter directories
    let replacestring = format!("{}/m/vim/", &config.home);
    let items = files
        .iter()
        .map(|x| x.replace(&replacestring, ""))
        .collect::<Vec<_>>();
    // check for arg0
    if let Some(arg0) = opt.file {
        // check arg0 matches files found in directory
        let target = items
            .into_iter()
            .filter(|i| i.to_string() == arg0)
            .collect::<Vec<_>>();
        // error handling whether match was found
        let isfound = match target.get(0) {
            Some(i) => i,
            None => &arg0,
        };
        if let Err(err) = openfile(&config.home, &config.editor, isfound) {
            println!("Error opening file: {}", err);
            process::exit(1);
        }
    }
    //}}}
}
// }}}
// read file {{{
fn read(home: &str, csv: &mut Vec<((String, String))>) -> Result<(), Error> {
    let path = format!("{}{}", home, "/m/abel/list");
    let file = fs::File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.deserialize() {
        let record: Csv = result?;
        csv.push((record.alias, record.dir))
    }
    Ok(())
}
// }}}
// paths {{{
fn paths(home: &str, files: &mut Vec<String>) -> Result<(), Error> {
    let dir = format!("{}{}", home, "/m/vim");
    let paths = fs::read_dir(dir)?;
    for path in paths {
        files.push(path?.path().to_str().unwrap().to_string());
    }
    Ok(())
}
// }}}
// openFile {{{
fn openfile(home: &str, editor: &str, file: &str) -> Result<(), Error> {
    let location = format!("{}{}/{}", home, "/m/vim", file);
    let cmd = format!("{} {}", editor, location);
    println!("{:?}", &cmd);
    let args = &["-c", &cmd];
    duct::cmd("bash", args).run()?;
    Ok(())
}
// }}}
