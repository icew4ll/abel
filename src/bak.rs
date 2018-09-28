// imports {{{
#[macro_use]
extern crate envconfig_derive;
extern crate duct;
extern crate envconfig;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
// #[macro_use]
// extern crate lazy_static;
extern crate csv;
extern crate chrono;
// extern crate duct;
use chrono::prelude::*;
use duct::cmd;
use envconfig::Envconfig;
use std::error::Error;
use std::fs::File;
use std::process;
use structopt::StructOpt;
// macro to create vector of strings
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
// }}}
// structs and enums {{{
#[derive(Envconfig)]
// Config struct for env vars
struct Config {
    #[envconfig(from = "HOME")]
    home: String,
}
// csv struct from file
#[derive(Debug, Deserialize)]
struct Csv {
    alias: String,
    dir: String,
}
// index 0 parsing
#[derive(Debug, StructOpt)]
struct Opt {
    alias: Option<String>,
    #[structopt(subcommand)]
    cmd: Option<Command>,
}
// subcommand parsing
#[derive(Debug, StructOpt)]
enum Command {
    Push { alias: Option<String> },
    Mvq { ips: String },
    Rex { test: String },
}
// }}}
// main {{{
fn main() {
    // read csv {{{
    // Initialize config from environment variables
    let config = Config::init().unwrap_or_else(|err| {
        eprintln!("{}", err);
        ::std::process::exit(1);
    });
    // init vec to store csv data
    let mut csv = vec![];
    if let Err(err) = read(&config.home, &mut csv) {
        println!("error running example: {}", err);
        process::exit(1);
    }
    //}}}
    let opt = Opt::from_args();
    // println!("{:?}", opt.alias);
    // argument index 1 parsing {{{
    if let Some(arg1) = opt.alias {
        let t = &csv.into_iter().filter(|i| i.0 == arg1).collect::<Vec<_>>();
        // println!("{:?}", t);
        let alias = t[0].0.to_string();
        let dir = t[0].1.to_string();
        if let Err(err) = gitpush(&config.home, alias, dir) {
            println!("{}", err);
            process::exit(1);
        }
    }
    // }}}
    // match subcommands {{{
    match opt.cmd {
        Some(Command::Push { alias }) => {
            println!("{:?}", alias);
        }
        Some(Command::Mvq { ips }) => {
            println!("{:?}", ips);
        }
        _ => (),
    }
    // }}}
}
// }}}
// gitpush {{{
fn gitpush(home: &str, alias: String, dir: String) -> Result<(), Box<Error>> {
    println!("{} {} {}", home, alias, dir);
    let utc: DateTime<Utc> = Utc::now();
    let cmds = vec_of_strings![
        format!("cd {}{}", home, dir),
        "git add -A",
        format!("git commit -m '{}'", utc),
        "git push"
    ];
    println!("{}", &cmds.join(";"));
    let args = &["-c", &cmds.join(";")];
    cmd("bash", args).run().unwrap();
    Ok(())
}
// }}}
// read file {{{
fn read(home: &str, csv: &mut Vec<((String, String))>) -> Result<(), Box<Error>> {
    // println!("{}", home);
    let path = format!("{}{}", home, "/m/abel/list");
    let file = File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new().flexible(true).from_reader(file);
    for result in rdr.deserialize() {
        let record: Csv = result?;
        csv.push((record.alias, record.dir))
    }
    Ok(())
}
// }}}
