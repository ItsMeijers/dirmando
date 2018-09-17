extern crate clap;

use clap::{Arg, App};

fn main() {
    let matches = App::new("dirmando")
        .version("0.1.0")
        .author("Thomas Meijers <thomasmeijers@live.nl>")
        .about("Save and retrieve commands specific to directories.")
        .arg(Arg::with_name("choose")
            .long("choose")
            .short("c")
            .required(false)
            .takes_value(false)
            .help("Shows all the commands saved for this directory.")
            .conflicts_with_all(&vec!["save", "export", "import"]))
        .arg(Arg::with_name("save")
            .long("save")
            .short("s")
            .required(false)
            .takes_value(true)
            .help("Saves a command for the current directory.")
            .conflicts_with_all(&vec!["choose", "export", "import"]))
        .arg(Arg::with_name("export")
            .long("export")
            .short("e")
            .required(false)
            .takes_value(true)
            .help("Exports all the commands and directories to import on another device.")
            .conflicts_with_all(&vec!["save", "choose", "import"]))
        .arg(Arg::with_name("import")
            .long("import")
            .short("i")
            .required(false)
            .takes_value(true)
            .help("Imports all the commands and directories from an import file.")
            .conflicts_with_all(&vec!["save", "export", "choose"]))
        .get_matches();

    let url = matches.value_of("URL").unwrap();

    println!("{}", url);
}
