use std::path;
use std::fs;
use std::io;
use std::process::Command;

fn main() -> io::Result<()> {
    clear_headers()?;

    // generate new headers
    // cbindgen --config cbindgen.toml --crate rust --output librust.h --lang c
    Command::new("cbindgen")
        .args(&["--config", "cbindgen.toml"])
        .args(&["--crate", "rust"])
        .args(&["--output", "./headers/librust.h"])
        .args(&["--lang", "c"])
        .status()?;
    
    Ok(())
}

/// clears headers directory without deleting it from git
fn clear_headers() -> io::Result<()> {
    let headers_dir = path::PathBuf::from("./headers");
    for entry in fs::read_dir(headers_dir)? {
        let e = entry?;
        let tpe = e.file_type()?;
        if tpe.is_dir() {
            fs::remove_dir_all(e.path())?;
        } else {
            if e.file_name() == ".gitkeep" {
                continue;
            }
            fs::remove_file(e.path())?;
        }
    }
    Ok(())
}