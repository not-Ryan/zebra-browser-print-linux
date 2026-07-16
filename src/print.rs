use std::{
    io::Write,
    process::{Command, Stdio},
};

use cmd_lib::CmdResult;

pub fn print_label(device_name: &str, zpl: &str) -> CmdResult {
    let mut lp = Command::new("lp")
        .args(["-d", device_name, "-o", "raw", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    lp.stdin
        .as_ref()
        .unwrap()
        .write_all(zpl.as_bytes())
        .unwrap();

    lp.wait().unwrap();

    Ok(())
}
