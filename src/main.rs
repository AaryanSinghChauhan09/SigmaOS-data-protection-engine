//! SigmaOS-data-protection-engine — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_data_protection_engine_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-data-protection-engine: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-data-protection-engine: ok");
}
