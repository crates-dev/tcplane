/// Prints error message to stderr and flushes the buffer.
///
/// # Arguments
///
/// - `String` - Error message to be printed.
///
/// # Returns
///
/// - `()` - This function does not return any meaningful value.
pub(crate) fn print_error_handle(error: String) {
    eprintln!("{error}");
    let _ = std::io::Write::flush(&mut std::io::stderr());
}
