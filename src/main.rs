fn main() {
    if let Err(e) = pdet::get_args().and_then(pdet::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
