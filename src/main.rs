extern crate ffmpeg_next as ffmpeg;

use clap::{crate_authors, crate_description, crate_name, value_t, App, Arg};
use colored::Colorize;
use url::Url;

/// Intermediary FIFO path
const INTERMEDIARY_FIFO: &str = "/tmp/ittyr.stream.wav";

/// ITTY Streams
struct IttyStream<'a> {
    pub name: &'a str,
    pub url: Url,
    pub baud_mode: &'a str,
}

fn main() {
    let matches = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            Arg::with_name("stream")
                .default_value("default")
                .value_names(&["default", "europe", "autostart"])
                .help("The TTY stream to read from")
                .takes_value(true),
        )
        .get_matches();

    // Get the correct stream data
    let stream_data = match matches.value_of("stream") {
        Some(value) => match value {
            "europe" => Some(IttyStream {
                name: "Europe News",
                url: "http://internet-tty.net:8040/EUROPE".parse().unwrap(),
                baud_mode: "50 --baudot --stopbits=1.5",
            }),
            "autostart" => Some(IttyStream {
                name: "Autostart",
                url: "http://internet-tty.net:8030/AUTOSTART".parse().unwrap(),
                baud_mode: "rtty",
            }),
            "default" => Some(IttyStream {
                name: "ITTY Broadcast",
                url: "http://internet-tty.net:8000/ITTY".parse().unwrap(),
                baud_mode: "rtty",
            }),
            _ => None,
        },
        None => None,
    }
    .expect("Failed to parse \"stream\" argument");

    // Print the program start header
    println!("Selected stream: {}", stream_data.name.bright_blue());
    println!(
        "Upstream URL: {}",
        stream_data.url.to_string().bright_blue()
    );

    // Create an FFMPEG input for the stream
    let ff_input = ffmpeg::format::input(&stream_data.url.to_string()).unwrap();

    // Set up the stream FIFO
    if std::fs::metadata(INTERMEDIARY_FIFO).is_ok() {
        std::fs::remove_file(INTERMEDIARY_FIFO)
            .expect("Could not remove intermediary audio FIFO");
    }
}
