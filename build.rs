use ethers::contract::Abigen;
use std::fs::File;

const PATH: &str = "./abi/Liquidations.json";

// Generates the bindings under `src/`
fn main() {
    // Only re-run the builder script if the contract changes
    println!("cargo:rerun-if-changed={}", PATH);

    // generate type-safe bindings to it
    let bindings = Abigen::new("Liquidations", PATH)
        .expect("could not instantiate Abigen")
        .generate()
        .expect("could not generate bindings");

    bindings
        .write_to_file("./src/liquidation.rs")
        .expect("could not write bindings to file");
}
