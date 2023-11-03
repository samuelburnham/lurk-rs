use assert_cmd::prelude::*;
use camino::Utf8Path;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;
use tempfile::Builder;

fn lurk_cmd() -> Command {
    Command::cargo_bin("lurk").unwrap()
}

#[test]
fn test_help_subcommand() {
    let mut cmd = lurk_cmd();

    cmd.arg("help");
    cmd.assert().success();
}

#[test]
fn test_help_flag_command() {
    let mut cmd = lurk_cmd();

    cmd.arg("--help");
    cmd.assert().success();
}

#[test]
fn test_repl_command() {
    let mut cmd = lurk_cmd();

    cmd.arg("repl");
    cmd.assert().success();
}

#[test]
fn test_bad_command() {
    let tmp_dir = Builder::new().prefix("tmp").tempdir().unwrap();
    let bad_file = tmp_dir.path().join("uiop");

    let mut cmd = lurk_cmd();
    cmd.arg(bad_file.to_str().unwrap());
    cmd.assert().failure();
}

// Tests that commitments correctly persist on the file system between `lurk` invocations
#[test]
fn test_commit_and_open() {
    let tmp_dir = Builder::new().prefix("tmp").tempdir().unwrap();
    let tmp_dir = Utf8Path::from_path(tmp_dir.path()).unwrap();
    let commit_dir = tmp_dir.join("commits");
    let commit_file = tmp_dir.join("commit.lurk");
    let open_file = tmp_dir.join("open.lurk");

    let mut file = File::create(commit_file.clone()).unwrap();
    file.write_all(b"!(commit (+ 1 1))\n").unwrap();

    let mut file = File::create(open_file.clone()).unwrap();
    file.write_all(b"!(open 0x0973b895ba6991037628d9d3a5661152e034abcd459b61be9faa4b67304774c6)\n")
        .unwrap();

    let mut cmd = lurk_cmd();
    cmd.arg("load");
    cmd.arg(commit_file.into_string());
    cmd.arg("--commits-dir");
    cmd.arg(&commit_dir);
    assert_eq!(
        &String::from_utf8(cmd.output().unwrap().stdout)
            .unwrap()
            .lines()
            .collect::<Vec<String>>(),
        "Data: 2\nHash: 0x0973b895ba6991037628d9d3a5661152e034abcd459b61be9faa4b67304774c6"
    );
    //assert().success();

    let mut cmd = lurk_cmd();
    cmd.arg("load");
    cmd.arg(open_file.into_string());
    cmd.arg("--commits-dir");
    cmd.arg(commit_dir);
    cmd.assert().success();
}

// TODO: Use a snapshot test for the proof ID and/or test the REPL process
#[test]
fn test_prove_and_verify() {
    let tmp_dir = Builder::new().prefix("tmp").tempdir().unwrap();
    let tmp_dir = Utf8Path::from_path(tmp_dir.path()).unwrap();
    let public_param_dir = tmp_dir.join("public_params");
    let proof_dir = tmp_dir.join("proofs");
    let commit_dir = tmp_dir.join("commits");
    let lurk_file = tmp_dir.join("prove_verify.lurk");

    let mut file = File::create(lurk_file.clone()).unwrap();
    file.write_all(b"!(prove (+ 1 1))\n").unwrap();
    file.write_all(b"!(verify \"Nova_Pallas_10_3f2526abf20fc9006dd93c0d3ff49954ef070ef52d2e88426974de42cc27bdb2\")\n").unwrap();

    let mut cmd = lurk_cmd();
    cmd.arg("load");
    cmd.arg(lurk_file.into_string());
    cmd.arg("--public-params-dir");
    cmd.arg(public_param_dir);
    cmd.arg("--proofs-dir");
    cmd.arg(proof_dir);
    cmd.arg("--commits-dir");
    cmd.arg(commit_dir);

    cmd.assert().success();
}

//#[test]
//fn test_chained_fcomm() {
//    let tmp_dir = Builder::new().prefix("tmp").tempdir().unwrap();
//    let commit_dir = tmp_dir.join("commits");
//
//    let mut cmd = lurk_cmd();
//    cmd.arg("commit");
//    cmd.assert().failure();
//}
