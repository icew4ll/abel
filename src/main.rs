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
// use duct::cmd;
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
    // read csv {{{
    // Initialize config from environment variables
    let config = Config::init().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });
    if let Err(err) = read(&config.home, &mut csv) {
        println!("error running example: {}", err);
        process::exit(1);
    }
    if let Err(err) = paths(&config.home, &mut files) {
        println!("Error gettings paths {}", err);
        process::exit(1);
    }
    // filter directories
    let items = files
        .iter()
        .map(|x| x.replace("/home/fish/m/vim/", ""))
        .collect::<Vec<_>>();
    // check for arg0
    if let Some(arg0) = opt.file {
        println!("{}", arg0);
        let target = items
            .into_iter()
            .filter(|i| i.to_string() == arg0)
            .collect::<Vec<_>>();
        println!("match {}", target.get(0).expect("Match not found"));
    }
    // &items.iter().filter(|x| println!("{:?}", x);
    // println!("{:?}", &items;
    //
// mailq.iter().map(|x| &x.1).for_each(|x| println!("{:?}", x));
// let q = mailq.iter().map(|x| &x.1).collect::<Vec<_>>();
    //}}}
}
// }}}
// read file {{{
fn read(home: &str, csv: &mut Vec<((String, String))>) -> Result<(), Error> {
    // println!("{}", home);
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
// paths
fn paths(home: &str, files: &mut Vec<String>) -> Result<(), Error> {
    let dir = format!("{}{}", home, "/m/vim");
    let paths = fs::read_dir(dir)?;
    for path in paths {
        // println!("{}", path?.path().display())
        files.push(path?.path().to_str().unwrap().to_string());
    }
    Ok(())
}
// }}}
