use assert_cmd::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn compiler_sanity_check() -> Result<(), Box<dyn std::error::Error>> {
    // List all .bf files in files
    let mut tests_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    tests_dir.push("tests");
    tests_dir.push("files");
    let bf_files: Vec<PathBuf> = fs::read_dir(tests_dir)?
        .map(|res| res.map(|e| e.path()))
        .filter_map(|ele| match ele {
            Ok(x) => Some(x),
            Err(_) => None,
        })
        .filter(|x| x.extension().unwrap() == "bf")
        .collect();

    // Run all .bf files and compare outputs
    for bf_file in bf_files {
        let mut out_file = bf_file.clone();
        out_file.set_extension("stdout");
        let expected_stdout = fs::read_to_string(out_file).unwrap();
        let mut cmd = Command::cargo_bin("brainfk-rs")?;
        cmd.args([
            "run",
            bf_file.as_os_str().to_str().unwrap(),
            "--backend",
            "llvm",
        ]);
        cmd.assert().success().stdout(expected_stdout);
    }
    Ok(())
}
