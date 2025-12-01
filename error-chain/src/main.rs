use anyhow::Context;
use thiserror::Error;

fn call1() -> Result<(), anyhow::Error> {
    call2().context("error1")
}

fn call2() -> Result<(), anyhow::Error> {
    call3().context("error2")
}

fn call3() -> Result<(), anyhow::Error> {
    Err(anyhow::Error::msg("call3 error"))
}

fn main() {
    if let Err(e) = call1() {
        eprintln!("Error: {:?}", e); // Debug format shows full chain
    }
}
