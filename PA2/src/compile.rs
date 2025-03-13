use std::{error::Error, io, process::Command};

pub fn compile() -> Result<(), Box<dyn Error>> {
    let compile_status = Command::new("gcc")
        .arg("-shared")
        .arg("-o")
        .arg("c_src/pa2.so")
        .arg("-fPIC")
        .arg("c_src/pa2.c")
        .arg("-Wall")
        .arg("-Werror")
        .arg("-Og")
        .arg("-g")
        .status()?;

    if !compile_status.success() {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "gcc compilation failed",
        )));
    }

    Ok(())
}
