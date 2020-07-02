// use std::path;
// use std::fs;
use std::io;
// use std::process::Command;

fn main() -> io::Result<()> {
    // clear_dir("./pkg")?;

    // TODO for whatever reasons this does never finish. We'll have to investigate why
    // Compare:
    //  - https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm
    //  - https://rustwasm.github.io/wasm-bindgen/reference/deployment.html#bundlers
    // wasm-pack build --target nodejs
    // Command::new("wasm-pack")
    //     .args(&["build"])
    //     .args(&["--target", "nodejs"])
    //     .status()?;
    
    Ok(())
}

// /// clear_dir clears all content from the given path without removing it from git
// fn clear_dir(dir: &str) -> io::Result<()> {
//     let headers_dir = path::PathBuf::from(dir);
//     for entry in fs::read_dir(headers_dir)? {
//         let e = entry?;
//         let tpe = e.file_type()?;
//         if tpe.is_dir() {
//             fs::remove_dir_all(e.path())?;
//         } else {
//             if e.file_name() == ".gitkeep" {
//                 continue;
//             }
//             fs::remove_file(e.path())?;
//         }
//     }
//     Ok(())
// }