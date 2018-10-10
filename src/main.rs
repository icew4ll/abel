// imports {{{
extern crate failure;
// extern crate glob;
#[macro_use]
extern crate envconfig_derive;
extern crate duct;
extern crate envconfig;
// #[macro_use]
extern crate structopt;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate csv;
use chrono::prelude::*;
use envconfig::Envconfig;
use failure::Error;
// use glob::glob;
use std::fs;
use std::process;
use structopt::StructOpt;
// use std::io::{self};
// use structopt::StructOpt;
// macro to create vector of strings
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}
// }}}
// structs and enums {{{
// environment variables
#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "HOME")]
    home: String,
    #[envconfig(from = "EDITOR")]
    editor: String,
}
// csv columns
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
    sub: Option<Sub>,
}
// subcommand parsing
#[derive(Debug, StructOpt)]
enum Sub {
    Push { repo: String },
    Mvq { ips: String },
    Rex { test: String },
}
// }}}
// main {{{
fn main() {
    // Setup data storage
    let opt = Opt::from_args();
    let mut csv = vec![];
    let mut files = vec![];
    // Get environment variables
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
    // filter files from paths
    let replacestring = format!("{}/m/vim/", &config.home);
    let items = files
        .iter()
        .map(|x| x.replace(&replacestring, ""))
        .collect::<Vec<_>>();
    // check if argument index 0 exists
    if let Some(arg0) = opt.file {
        // check arg0 matches files found in directory
        let target = items.into_iter().filter(|i| i == &arg0).collect::<Vec<_>>();
        // Error handling: if target.get(0) exists, store in isfound else store arg0
        let isfound = match target.get(0) {
            Some(i) => i,
            None => &arg0,
        };
        if let Err(err) = openfile(&config.home, &config.editor, isfound) {
            println!("Error opening file: {}", err);
            process::exit(1);
        }
    }
    // check if subcommands exists
    if let Some(Sub::Push { repo }) = opt.sub {
        if let Err(err) = push(&config.home, &repo) {
            println!("{}", err);
            process::exit(1);
        }
    }
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
    let location = format!("{}/m/vim/{}", home, file);
    let cmd = format!("{} {}", editor, location);
    println!("{:?}", &cmd);
    let args = &["-c", &cmd];
    duct::cmd("bash", args).run()?;
    Ok(())
}
// }}}
// openFile {{{
fn push(home: &str, dir: &str) -> Result<(), Error> {
    let utc: DateTime<Utc> = Utc::now();
    let location = format!("{}/m/{}", home, dir);
    let cd = vec_of_strings![format!("cd {}", location)];
    let copy = vec_of_strings![
        format!(
            "cp {}/.config/alacritty/alacritty.yml {}/m/dot/",
            home, home
        ),
        format!("cp {}/.config/nvim/init.vim {}/m/dot/", home, home),
        format!("cp {}/.config/ion/initrc {}/m/dot/", home, home),
        format!("cp {}/.tmux.conf.local {}/m/dot/", home, home)
    ];
    let push = vec_of_strings![
        "sed -i 's/https:\\/\\/github.com\\//git@github.com:/' .git/config",
        "git add -A",
        format!("git commit -m '{}'", utc),
        "git push"
    ];
    let cmd = match dir {
        "dot" => [&cd[..], &copy[..], &push[..]].concat(),
        _ => [&cd[..], &push[..]].concat(),
    };
    println!("{:?}", &cmd);
    let args = &["-c", &cmd.join(";")];
    duct::cmd("bash", args).run()?;
    Ok(())
}
// }}}
