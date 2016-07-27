extern crate fsort;

use fsort::*;
use fsort::file_collection::FileCollection;

use std::io::Write;
use std::io::BufRead;
use std::ops::DerefMut;
use std::env::Args;

fn main() {
    // Reads in command line arguments and check if they are sufficient.
    if std::env::args().len() == 1 {
        show_usage();
        return;
    }

    let (mut sort, args) = parse_arguments();

    // If the paths are provided as arguments ...
    if args.len() > 0 {
        for path_string in args {
            add_path(&mut sort, &path_string);
        }
    } else {
        let stdin = std::io::stdin();
        for path_string in stdin.lock().lines().filter_map(|x| x.ok()) {
            add_path(&mut sort, &path_string);
        }
    };

    // Print out the result
    print_sorted(sort);
}

fn parse_arguments() -> (file_collection::DynamicCollection, Args) {

    // Parse the arguments
    let mut sort = file_collection::DynamicCollection::new::<criterion::FileName>();
    let mut order = fsort::Order::Ascending;
    let mut recursive = false;

    let mut args = std::env::args();
    let mut args_argument = std::env::args();

    args.next().expect("Name missing!");
    args_argument.next().expect("Name missing!");

    while let Some(value) = args_argument.next() {
        match value.to_lowercase().trim() {
            "help" | "--help" | "/help" => {
                show_usage();
                std::process::exit(0);
            },
            "--s" | "--sort" | "/s" | "/sort" => {
                args.next();
                args.next();
                sort = match args_argument.next().as_ref().map(|x| x.as_ref()) {
                    Some("NAME") | Some("name") => file_collection::DynamicCollection::new::<criterion::FileName>(),
                    Some("CREATION") | Some("creation") => file_collection::DynamicCollection::new::<criterion::CreationDate>(),
                    Some("ACCESS") | Some("access") => file_collection::DynamicCollection::new::<criterion::AccessDate>(),
                    Some("MODIFY") | Some("modify") => file_collection::DynamicCollection::new::<criterion::ModifyDate>(),
                    Some("SIZE") | Some("size") => file_collection::DynamicCollection::new::<criterion::FileSize>(),
                    _ => fail("Invalid / missing criterion for sorting!")
                };
            },
            "--o" | "--order" | "/o" | "/order" => {
                args.next();
                args.next();
                order = match args_argument.next().as_ref().map(|x| x.as_ref()) {
                    Some("ASC") | Some("asc") => fsort::Order::Ascending,
                    Some("DESC") | Some("desc") => fsort::Order::Descending,
                    _ => fail("Invalid / missing order for sorting!")
                };
            },
            "--r" | "--recursive" | "/r" | "/recursive" => {
                args.next();
                recursive = true;
            },
            _ => { break }
        };
    }

    sort.set_order(order);
    sort.set_recursive(recursive);

    (sort, args)
}

fn show_usage() {
    let name = std::env::args().next().expect("Programname missing!");

    println!("fsort {} by Christopher Gundler", env!("CARGO_PKG_VERSION"));
    println!("USAGE: {} --help: This help", name);
    println!("USAGE: {} {{ Argument | Argument ' ' Value }} {{ Files / Directories }}", name);
    println!("Arguments: ");
    println!("\t --s[ort]: The criterion for the search");
    println!("\t\t NAME: Sort by the name (default)");
    println!("\t\t CREATION: Sort by creation date");
    println!("\t\t ACCESS: Sort by last access date");
    println!("\t\t MODIFY: Sort by modify date");
    println!("\t\t SIZE: Sort by the size");
    println!("\t --o[rder]: The order of the output");
    println!("\t\t ASC: Ascending output (default)");
    println!("\t\t DESC: Descending output");
    println!("\t --r[ecursive]: Should directory be visited recursively?");
    println!("Files / Directories: \tFiles to be sorted and/or directories which files are to be sorted.");
    println!("\t\t\tIf not specified, fsort reads from STDIN until EOF or whitespace.");
}

fn print_sorted(sorted : file_collection::DynamicCollection) {
    let mut stdout = std::io::stdout();
    for file in sorted {
        stdout.write_fmt(format_args!("{}\n", &std::path::PathBuf::from(file).to_string_lossy())).expect("Writing failed!");
    }
    stdout.flush().expect("Flushing failed!");
}

fn add_path(sort : &mut file_collection::DynamicCollection, path_string : &str) {
    let path = std::path::Path::new(path_string);
    if path.is_dir() {
        sort.add_directory(path, fsort::file_collection::OnError::Ignore);
    }
    else if path.is_file() {
        if let fsort::file_collection::Result::FileError(_, mut description) = sort.add_file(path) {
            print_err(&format!("Adding '{}' failed ({})!", path_string, description.pop().expect("Single error")));
        }
    }
    else {
        print_err(&format!("'{}' is not a valid path!", path_string));
    }
}

fn print_err(message : &str) {
    std::io::stderr().write_fmt(format_args!("[ERROR] {}\n", message)).expect("Writing to STDERR failed!");
}

fn fail(message : &str) -> ! {
    print_err(message);
    std::process::exit(1)
}