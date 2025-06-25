use std::error::Error;
use std::io;
use std::path::Path;
use std::sync::Mutex;
use slog::error;

use backtrace::Backtrace;
use serde::Serialize;
use slog::{o, Drain, Logger, Record, KV};
#[derive(Serialize, Debug)]
pub struct StackFrame {
    func: String,
    source: String,
    line: u32,
}

/// Attempts to extract a stack trace from an error.
/// In this simplified example, we capture the current backtrace.
fn marshal_stack(_err: &(dyn Error)) -> Option<Vec<StackFrame>> {
    let bt = Backtrace::new();
    let mut frames = Vec::new();

    // Iterate over the frames in the backtrace.
    for frame in bt.frames() {
        for symbol in frame.symbols() {
            let func = symbol
                .name()
                .map(|n| n.to_string())
                .unwrap_or_else(|| "unknown".to_owned());
            let source = symbol
                .filename()
                .and_then(|p| p.to_str())
                .map(|s| {
                    // Get only the last two components for brevity.
                    let path = Path::new(s);
                    if let (Some(parent), Some(file)) = (path.parent(), path.file_name()) {
                        format!("{}/{}", parent.file_name().unwrap_or_default().to_string_lossy(), file.to_string_lossy())
                    } else {
                        s.to_owned()
                    }
                })
                .unwrap_or_else(|| "unknown".to_owned());
            let line = symbol.lineno().unwrap_or(0);

            frames.push(StackFrame { func, source, line });
        }
    }
    if frames.is_empty() {
        None
    } else {
        Some(frames)
    }
}

/// Formats an error into an owned slog value that includes the error message and, if available, a stack trace.
/// Adds error details to a log record including the error message and stack trace if available.
pub fn fmt_err<'a>(err: &'a (dyn Error + 'a)) -> impl KV + 'a {
    let msg = err.to_string();
    if let Some(frames) = marshal_stack(err) {
        o!("error" => msg, "stack_trace" => format!("{:?}", frames))
    } else {
        o!("error" => msg, "stack_trace" => String::new())
    }
}

/// In this example, we do not dynamically replace attributes as in the Go version,
/// but you can call `fmt_err` when logging errors.
pub fn get_logger() -> Logger {
    let drain = slog_json::Json::default(io::stdout()).fuse();
    let drain = Mutex::new(drain).fuse();
    Logger::root(drain, o!())

}
pub fn error_context<E: std::error::Error + 'static>(logger: &Logger, context: &str, err: E) {
    error!(logger, "{}: {}", context, err);
}