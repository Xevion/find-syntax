use std::{path::{Path, PathBuf}, io, env, fs::create_dir_all};

use clap::Parser;
use log::{debug, error, info, trace, warn};

use path_clean::PathClean;

mod languages;
mod markdown;

pub fn absolute_path(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir().expect("").join(path)
    }.clean();

    absolute_path
}


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg()]
    file: String,

    #[arg(short, long, default_value = "./.cache")]
    cache_dir: String,
}

fn main() {
    let args = Args::parse();
    env_logger::init();

    let cache_dir = absolute_path(Path::new(&args.cache_dir));
    debug!("Cache directory: {}", cache_dir.display());

    if !cache_dir.exists() {
        debug!("Cache directory does not exist; creating...");
        create_dir_all(cache_dir).unwrap();
    }

    // TODO: Given an input file argument, read the file.
    let file = absolute_path(Path::new(&args.file));
    debug!("File: {}", file.display());

    if !file.exists() {
        error!("File does not exist: {}", file.display());
        return;
    }

    // Function to generate a formatted markdown document of the file.
    let get_formatted_file = |language: &str, content: &str| -> String {
        format!("```{language}\n{content}\n```")
    };


    // Determine all available languages for highlighting abilities.
    let languages = languages::get_languages().unwrap();
    debug!("{} unique languages found.", languages.len());

    // TODO: Begin querying GitHub Markdown API
    languages.iter().for_each(|language| {
        let markdown = markdown::get_markdown(&get_formatted_file(language, "Hello, world!")).unwrap();
        debug!("Markdown: {}", markdown);
    });
    

    // TODO: Cache responses locally
    // TODO: Determine complexity & level of colorization for each response
    // TODO: Provide a report of what syntax highlighting works best
}
