fn main() {
    // Result type has and_then method we can use to pass config returned
    // straight to another function
    if let Err(e) = catr::get_args().and_then(catr::run) {
        // just like print! macro but prints to stderr
        eprint!("{}", e);
        std::process::exit(1);
    }
}
