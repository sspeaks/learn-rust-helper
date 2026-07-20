fn main() {
    if let Err(err) = xtask::run() {
        if !matches!(err, xtask::XtaskError::CheckFailed) {
            eprintln!("error: {err}");
        }
        std::process::exit(1);
    }
}
