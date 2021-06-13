use std::process::{Child, Command, Stdio};

use colored::Colorize;
use nix::{sys::stat, unistd::mkfifo};

use crate::sources::IttyStream;

/// Intermediary FIFO path
pub const INTERMEDIARY_FIFO: &str = "/tmp/ittyr.stream.wav";

/// Spawn a transcoder process
pub fn spawn_transcoder(stream: &IttyStream) -> Child {
    // Set up the stream FIFO
    if std::fs::metadata(INTERMEDIARY_FIFO).is_ok() {
        eprintln!("{}", "Removing old FIFO...".bright_black());
        std::fs::remove_file(INTERMEDIARY_FIFO).expect("Could not remove intermediary audio FIFO");
    }

    // Create a new FIFO
    eprintln!("{}", "Creating intermediary FIFO...".bright_black());
    mkfifo(INTERMEDIARY_FIFO, stat::Mode::S_IRWXU).expect("Could not create intermediary FIFO");

    // Config transcoder process
    let mut proc = Command::new("ffmpeg");
    let proc = proc
        .arg("-y")
        .arg("-i")
        .arg(stream.url.to_string())
        .arg("-r")
        .arg("8")
        .arg("-preset")
        .arg("ultrafast")
        .arg("-fflags")
        .arg("nobuffer")
        .arg(INTERMEDIARY_FIFO)
        .stderr(Stdio::null())
        .stdout(Stdio::null());

    // Spawn the process
    proc.spawn()
        .expect("Failed to spawn an FFMPEG process. Is it installed and on the PATH?")
}
