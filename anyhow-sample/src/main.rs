use anyhow::{Context, Error, Result, bail};

fn func1() -> Result<()> {
    bail!("error1")
}

fn func2() -> Result<(), Error> {
    func1().context("func 2 error")?;
    Ok(())
}

fn func3() -> Result<(), Error> {
    func2().context("func 3 error")?;
    Ok(())
}

fn main() {
    let result = func3();
    match result {
        Ok(()) => println!("ok"),
        Err(e) => {
            // In error với full chain
            eprintln!("Error: {}", e);

            // In full chain (giống Go's errors.Unwrap)
            eprintln!("\n--- Error chain ---");
            for (i, cause) in e.chain().enumerate() {
                eprintln!("  {}: {}", i, cause);
            }
        }
    }
}
