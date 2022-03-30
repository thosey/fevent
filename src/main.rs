#[derive(Debug)]
struct Arguments {
    root: String,
}

use text_colorizer::*;

fn print_usage() {
    eprintln!(
        "{} - listen to changes in a specified folder",
        "fevent".green()
    );
    eprintln!("{}: fevent {}", "Usage".blue(), "<root>".blue());
}

use std::env;

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        print_usage();
    }
    Arguments {
        root: args[0].clone(),
    }
}

use notify::Event;
use notify::EventKind;
use std::string::String;
fn print_event(evt: Event) {
    for path in evt.paths.iter() {
        let mut str = String::new();
        match evt.kind {
            EventKind::Create(_) => {
                str.push_str(" + ");
            }
            EventKind::Remove(_) => {
                str.push_str(" - ");
            }
            EventKind::Modify(_) => {
                str.push_str(" M ");
            }
            _ => {
                continue;
            }
        }
        println!("{}{:?}",str, path);
    }
}

use notify::{RecursiveMode, Result, Watcher};
use std::path::Path;
use std::{thread, time};
fn main() -> Result<()> {
    let args = parse_args();
    let mut watcher = notify::recommended_watcher(|res| match res {
        Ok(event) => print_event(event),
        Err(e) => eprintln!("watch error: {:?}", e),
    })?;
    watcher.watch(Path::new(&args.root), RecursiveMode::Recursive)?;
    loop {
        thread::sleep(time::Duration::from_secs(1));
    }
}
