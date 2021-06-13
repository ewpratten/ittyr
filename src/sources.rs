use url::Url;

/// ITTY Streams
#[derive(Clone, Debug)]
pub struct IttyStream<'a> {
    pub name: &'a str,
    pub url: Url,
    pub baud_mode: &'a str,
}

/// Create a stream object from a string describing it
pub fn stream_from_str<'a>(value: &str) -> Option<IttyStream<'a>> {
    match value {
        "europe" => Some(IttyStream {
            name: "Europe News",
            url: "http://internet-tty.net:8040/EUROPE".parse().unwrap(),
            baud_mode: "50 --baudot --stopbits=1.5",
        }),
        "autostart" => Some(IttyStream {
            name: "Autostart",
            url: "http://internet-tty.net:8030/AUTOSTART".parse().unwrap(),
            baud_mode: "rtty -5",
        }),
        "default" => Some(IttyStream {
            name: "ITTY Broadcast",
            url: "http://internet-tty.net:8000/ITTY".parse().unwrap(),
            baud_mode: "rtty -5",
        }),
        _ => None,
    }
}
