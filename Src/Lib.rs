use pyo3::prelude::*;
use regex::Regex;

/// Evaluates a command string against deterministic rules.
/// Returns `true` if safe, `false` if blocked.
#[pyfunction]
fn evaluate_command(command: &str) -> PyResult<bool> {
    // Ultra-fast regex evaluation directly on string slice (&str)
    let dangerous_pattern = match Regex::new(r"(?i)(rm\s+-rf|shred|busybox|cat\s+/etc/passwd|> /dev/sd)") {
        Ok(re) => re,
        Err(_) => return Ok(false),
    };

    if dangerous_pattern.is_match(command) {
        Ok(false) // Blocked execution
    } else {
        Ok(true)  // Passed security gate
    }
}

/// PyO3 Python Module Export
#[pymodule]
fn ldn_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(evaluate_command, m)?)?;
    Ok(())
}
