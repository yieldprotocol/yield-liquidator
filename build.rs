use ethers::contract::Abigen;

// TODO: Figure out how to write the rerun-if-changed script properly
fn main() {
    // Only re-run the builder script if the contract changes
    println!("cargo:rerun-if-changed=./abi/*.json");

    // bindgen("Treasury");
    // bindgen("WETH");
    // bindgen("YDai");
    // bindgen("Vat");
    // bindgen("Liquidations");
    // bindgen("Controller"); // TODO: Support I256 in ethers-rs
}

#[allow(dead_code)]
fn bindgen(fname: &str) {
    let bindings = Abigen::new(fname, format!("./abi/{}.json", fname))
        .expect("could not instantiate Abigen")
        .generate()
        .expect("could not generate bindings");

    bindings
        .write_to_file(format!("./src/bindings/{}.rs", fname.to_lowercase()))
        .expect("could not write bindings to file");
}
