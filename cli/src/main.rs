use std::env;
use std::error::Error;
use std::path::Path;
use std::process::exit;
use std::time::Duration;

use notify::{RecursiveMode, Watcher};
use notify_debouncer_full::new_debouncer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        actfv::show_usage();
        exit(1);
    } else {
        let source_file_path = &args[1];
        let target_file_path = &args[2];
        let profile = &args[3];

        // TODO replace this when I switch it to clap, and can pass a watcher param in
        if args.len() == 5 {
            watch(source_file_path, target_file_path, profile)?;
        } else {
            actfv::parse_and_write(source_file_path, target_file_path, profile)?;
        }
    }
    Ok(())
}

fn watch(
    source_file_path: &String,
    target_file_path: &String,
    profile: &String,
) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut debouncer = new_debouncer(Duration::from_secs(2), None, tx)?;

    debouncer
        .watcher()
        .watch(source_file_path.as_ref(), RecursiveMode::NonRecursive)?;

    debouncer.cache().add_root(
        <String as AsRef<Path>>::as_ref(source_file_path),
        RecursiveMode::NonRecursive,
    );

    for res in rx {
        match res {
            Ok(_) => {
                println!("Source file changed, updating target file.");
                actfv::parse_and_write(source_file_path, target_file_path, profile)?;
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}
