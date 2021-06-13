use std::process::{Child, Command, Stdio};

use crate::{sources::IttyStream, transcode::INTERMEDIARY_FIFO};

/// Spawn a decoder process
pub fn spawn_decoder(stream: &IttyStream) -> Child {
    // Config decoder process
    let mut proc = Command::new("minimodem");
    let proc = proc
        .args(stream.baud_mode.split(' '))
        .arg("-a")
        .arg("-i")
        .arg("--file")
        .arg(INTERMEDIARY_FIFO)
        .stderr(Stdio::null())
        .stdout(Stdio::piped());

    // Spawn the process
    proc.spawn()
        .expect("Failed to spawn an minimodem process. Is it installed and on the PATH?")
}
