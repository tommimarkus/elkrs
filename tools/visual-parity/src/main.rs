fn main() {
    if let Err(error) = elkrs_visual_parity::run(std::env::args().skip(1)) {
        eprintln!("{error}");
        std::process::exit(1);
    }
}
