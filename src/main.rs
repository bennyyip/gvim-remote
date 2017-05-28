#![windows_subsystem = "windows"]
use std::process::Command;
use std::env;

fn main() {
    Command::new("gvim")
        .arg("--remote-tab-silent")
        .args(env::args().skip(1))
        .spawn()
        .expect("gvim failed to start");
}
