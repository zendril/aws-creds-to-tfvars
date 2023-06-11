use std::env;
use std::error::Error;
use std::path::Path;
use std::process::exit;
use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};


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
            //use watcher
            watch(source_file_path, target_file_path, profile)?;
            // parse_and_write(source_file_path, target_file_path, profile)?;
        } else {
            // just run once
            parse_and_write(source_file_path, target_file_path, profile)?;
        }

    }
    Ok(())
}

fn parse_and_write(source_file_path: &String, target_file_path: &String, profile: &String) -> Result<(), Box<dyn Error>> {
    let source_map = actfv::parse_source(source_file_path).unwrap();
    println!("{:?}", source_map);
    let entries = actfv::get_entries_for_profile(source_map, profile)?;
    actfv::write_target(entries, target_file_path)?;
    Ok(())
}


fn watch(source_file_path: &String, target_file_path: &String, profile: &String) -> Result<(), Box<dyn Error>> {
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(source_file_path.as_ref(), RecursiveMode::NonRecursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                println!("changed: {:?}", event);
                parse_and_write(source_file_path, target_file_path, profile)?;
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}