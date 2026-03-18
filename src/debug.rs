use std::fs::OpenOptions;
use std::io::Write;

const DEBUG_LOG: &str = "/tmp/tmuxgotchi-debug.log";

pub fn log(msg: &str) {
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(DEBUG_LOG)
    {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs_f64())
            .unwrap_or(0.0);
        let _ = writeln!(file, "[{:.3}] {}", timestamp, msg);
    }
}

#[macro_export]
macro_rules! debug_log {
    ($($arg:tt)*) => {
        $crate::debug::log(&format!($($arg)*))
    };
}
