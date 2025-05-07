use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use counts::{format_dir_pattern, read_office_counts, CharCount};
use glob::glob;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(version = "0.1.0", about, long_about = None)]
struct Cli {
    /// target directory
    dir: String,
}

fn run(pattern: String) -> Result<()> {
    let start_time = Instant::now();
    let paths = glob(&pattern)?.flatten();
    let mut total_ct = CharCount::default();
    for path in paths {
        let ct = read_office_counts(&path)?;
        total_ct.merge(&ct);
    }
    let end_time = start_time.elapsed();

    println!();
    println!("{}", total_ct.get_display_str().green().bold());
    println!();
    println!(
        "{}",
        format!("总计用时: {}ms", end_time.as_millis())
            .blue()
            .bold()
    );

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let pattern = format_dir_pattern(args.dir)?;
    run(pattern)?;
    Ok(())
}
