//! This is a simple command line tool that caches the output of a command for a given TTL.

use clap::Parser;
use fclicache::{cache_aware_execute_command, hash};
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, author, name = "fclicache")]
struct Args {
    #[arg(short, long, default_value = "3600")]
    ttl: u64,

    /// Clean a cache and re-execute the command.
    /// Of-cause the result will be cached again.
    #[arg(short = 'c', long = "clean")]
    force_renew_cache: bool,

    /// Target cli command to cache.
    /// This argument should be quoted if it contains spaces.
    /// For example, 'sleep 10 && date'
    command: String,
}

fn main() {
    let args = Args::parse();
    let ttl = args.ttl;
    let command: String = args.command;
    let does_force_renew_cache = args.force_renew_cache;

    let cache_root_dir = env::temp_dir().join("fclicache/caches");
    if !cache_root_dir.exists() {
        std::fs::create_dir_all(&cache_root_dir).unwrap_or_else(|_| {
            panic!(
                "Unable to create cache root directory: {:?}",
                cache_root_dir
            )
        });
    }

    print!(
        "{}",
        cache_aware_execute_command(
            &command,
            ttl,
            &cache_root_dir.join(hash(&command).to_string()),
            does_force_renew_cache,
        )
    );
}
