use nih_plug_xtask::*;

fn main() {
    match nih_plug_xtask::main() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}