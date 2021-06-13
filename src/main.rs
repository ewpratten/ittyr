use std::{
    io::{Read, Write},
    time::Duration,
};

use clap::{crate_authors, crate_description, crate_name, App, Arg};
use colored::Colorize;

use crate::{decode::spawn_decoder, sources::stream_from_str, transcode::spawn_transcoder};

mod decode;
mod sources;
mod transcode;

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
        .arg(
            Arg::with_name("delay")
                .short("d")
                .long("char-delay")
                .help("The amount of delay between each character printed to the screen in ms")
                .takes_value(true)
                .default_value("50"),
        )
        .arg(
            Arg::with_name("jitter")
                .short("j")
                .long("char-jitter")
                .help("The maximum variation in delay between each character in ms")
                .takes_value(true)
                .default_value("100"),
        )
        .get_matches();

    // Get the delay and jitter data
    let char_delay = Duration::from_millis(
        matches
            .value_of("delay")
            .expect("No \"delay\" specified!")
            .parse()
            .expect("\"delay\" argument is not an integer"),
    );
    let char_jitter = Duration::from_millis(
        matches
            .value_of("jitter")
            .expect("No \"jitter\" specified!")
            .parse()
            .expect("\"jitter\" argument is not an integer"),
    );

    // Get the correct stream data
    let stream_data = stream_from_str(
        matches
            .value_of("stream")
            .expect("Failed to parse \"stream\" argument"),
    )
    .unwrap();

    // Print the program start header
    eprintln!("Selected stream: {}", stream_data.name.bright_blue());
    eprintln!(
        "Upstream URL: {}",
        stream_data.url.to_string().bright_blue()
    );

    // Spawn the transcoder process
    let _transcoder_proc = spawn_transcoder(&stream_data);

    // Print a seperator
    eprintln!("{}", "-------- Begin Feed --------".bright_black());

    // Spawn the decoder process
    let decoder_proc = spawn_decoder(&stream_data);
    let mut decoded_output = decoder_proc.stdout.unwrap();

    loop {
        // Grab the next char to send to the screen
        let mut buffer = vec![0u8; 1];
        decoded_output
            .read_exact(&mut buffer)
            .expect("Could not read data from decoder");

        // Sleep the correct amount
        if char_delay.as_millis() > 0 {
            std::thread::sleep(
                char_delay
                    + Duration::from_millis(
                        (char_jitter.as_millis() as f32 * rand::random::<f32>()) as u64,
                    ),
            )
        }

        // Print the char
        print!("{}", buffer[0] as char);

        // Flush the output
        std::io::stdout().flush().unwrap();
    }
}
