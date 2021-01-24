// This Source Code Form is subject to the terms of the Mozilla Public License,
// v. 2.0. If a copy of the MPL was not distributed with this file, You can
// obtain one at https://mozilla.org/MPL/2.0/.
use ds_store::DSStore;

use std::env;
use std::error;
use std::fs::File;
use std::io::Read;
use std::process;

const PROGRAM_NAME: &str = "dsdump";
const PROGRAM_VERSION: &str = env!("CARGO_PKG_VERSION");
const PROGRAM_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
const PROGRAM_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const COPYRIGHT_YEAR: u16 = 2020;

enum Help {
    Long,
    Short,
}

fn main() {
    if let Err(e) = app() {
        println!("{}: {}", PROGRAM_NAME, e);
        process::exit(1);
    }
}

fn help(v: Help) {
    match v {
        Help::Short => println!(
            "usage: {} [-h | --help] [-v | --version] [INPUT]",
            PROGRAM_NAME
        ),
        Help::Long => {
            println!(
                "{} v{} (c) {} {}
{}

usage:
    {0} [options] [INPUT]

options:
    -h             show short help
    --help s       how this help
    -v | --version show version information",
                PROGRAM_NAME, PROGRAM_VERSION, PROGRAM_AUTHORS, COPYRIGHT_YEAR, PROGRAM_DESCRIPTION,
            );
        }
    }
    process::exit(0);
}

fn version() {
    println!(
        "{} v{} (c) {} {}",
        PROGRAM_NAME, PROGRAM_VERSION, PROGRAM_AUTHORS, COPYRIGHT_YEAR
    );
    process::exit(0);
}

fn app() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args[1] == "-h" {
        help(Help::Short);
    }

    match args[1].as_str() {
        "--help" => help(Help::Long),
        "-v" | "--version" => version(),
        _ => {}
    }

    let mut f = File::open(&args[1])?;
    let mut data = Vec::new();
    f.read_to_end(&mut data)?;
    let ds_store = DSStore::new(data).parse()?;

    println!("freelist:");
    for (key, vals) in ds_store.free_list {
        if vals.len() == 0 {
            continue;
        }

        println!("  {:#x}:", key);
        for val in vals {
            println!("    {:#x}", val);
        }
    }

    println!("\ntable of contents: ");
    for (key, val) in ds_store.table_of_contents {
        println!("  {}:\n    {:#x}", key, val);
    }

    println!("\noffsets: ");
    for offset in ds_store.offsets {
        println!("  {:#x}", offset);
    }

    println!("\nrecords: ");
    for record in ds_store.records {
        println!(" {:#x?}", record);
    }

    Ok(())
}
