#![doc = include_str!("../README.md")]

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::process::Command;

pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    t.hash(&mut hasher);
    hasher.finish()
}

/// This function executes the given command and caches the result if the cache is expired or not exists.
/// If the cache is not expired, it just returns the cached result without execution command.
pub fn cache_aware_execute_command(command: &str, ttl: u64, cache_file: &PathBuf) -> String {
    if cache_file.exists() && cache_file.is_file() {
        let metadata = fs::metadata(cache_file).expect("Unable to read metadata of cache file");
        let created = metadata
            .created()
            .expect("Unable to read created date of cache file");
        let now = std::time::SystemTime::now();
        let elapsed = now
            .duration_since(created)
            .expect("Unable to calculate elapsed time");
        if elapsed.as_secs() < ttl {
            return String::from_utf8_lossy(
                &fs::read(cache_file).expect("Unable to read cache file"),
            )
            .to_string();
        }
    }

    let output = Command::new("sh")
        .args(["-c", command])
        .output()
        .expect("failed to execute process");
    fs::write(cache_file, &output.stdout).expect("Unable to write cache file");
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        thread::sleep,
        time::{Duration, Instant},
    };

    /// TestContext is a helper struct
    /// to create a unique cache directory for each test
    /// and remove it when the test is finished.
    struct TestContext {
        cache_root_path: std::path::PathBuf,
    }

    impl TestContext {
        /// Create a new test context with a unique string.
        /// The unique string is used to create a unique cache directory for each test.
        /// Because the cache directory is removed when the test context is dropped and
        /// multiple test cases is running in parallel, the cache directory should be unique.
        fn new(unique_string: &str) -> Self {
            let cache_root_path = std::env::temp_dir().join(format!("fclicache/{}", unique_string));
            fs::create_dir_all(&cache_root_path).expect("Unable to create cache directory");

            TestContext { cache_root_path }
        }
    }

    impl Drop for TestContext {
        /// Remove the cache directory when the test context is dropped.
        fn drop(&mut self) {
            fs::remove_dir_all(&self.cache_root_path).expect("Unable to remove cache directory");
        }
    }

    #[test]
    fn just_return_cache_without_execution_if_cache_is_exists() {
        let ctx = TestContext::new(&format!("{}{}", file!(), line!()));

        let cache_file = ctx.cache_root_path.join("test_cache");
        let _ = std::fs::write(&cache_file, "not hello").expect("Unable to write cache file");

        let command = "sleep 10 && echo 'hello'";
        let ttl = 60;

        let start = Instant::now(); // Start timing

        let result = super::cache_aware_execute_command(command, ttl, &cache_file);
        assert_eq!(result, "not hello");

        let duration = start.elapsed(); // Measure how long it took
        assert!(
            duration <= Duration::from_secs(5),
            "Test took too long: {:?}",
            duration
        );
    }

    #[test]
    fn execute_command_if_cache_does_not_exists() {
        let ctx = TestContext::new(&format!("{}{}", file!(), line!()));

        let cache_file = ctx.cache_root_path.join("test_cache");

        let command = "sleep 2 && echo 'hello'";
        let ttl = 60;

        let start = Instant::now(); // Start timing

        let result = super::cache_aware_execute_command(command, ttl, &cache_file);
        assert_eq!(result, "hello\n");

        let duration = start.elapsed(); // Measure how long it took
        assert!(
            duration >= Duration::from_secs(1),
            "Test took too short: {:?}",
            duration
        );
    }

    #[test]
    fn execute_command_if_cache_is_expired() {
        let ctx = TestContext::new(&format!("{}{}", file!(), line!()));

        let cache_file = ctx.cache_root_path.join("test_cache");
        let _ = std::fs::write(&cache_file, "not hello").expect("Unable to write cache file");

        let command = "sleep 2 && echo 'hello'";
        let ttl = 1;

        sleep(Duration::from_secs(1));
        let start = Instant::now(); // Start timing

        let result = super::cache_aware_execute_command(command, ttl, &cache_file);
        assert_eq!(result, "hello\n");

        let duration = start.elapsed(); // Measure how long it took
        assert!(
            duration >= Duration::from_secs(1),
            "Test took too short: {:?}",
            duration
        );
    }
}
