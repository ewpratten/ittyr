# Internet Teletype Reader
[![Crates.io](https://img.shields.io/crates/v/ittyr)](https://crates.io/crates/ittyr)

`ittyr` is a CLI [Internet Teletype](http://www.rtty.com/itty/index.htm) client that decodes and outputs ITTY messages in *real time* as they are being streamed through the ITTY service. The following streams are available:

 - ITTY Broadcast
 - Europe News
 - Autostart

In the backend, `ittyr` uses [`ffmpeg`](https://www.ffmpeg.org/) to transcode the ITTY audio stream to PCM audio data, then uses [`minimodem`](http://www.whence.com/minimodem/) to decode RTTY data into ASCII text.

## Installation

This program **only works on Linux**. You will need `ffmpeg` and `minimodem` (both can be downloaded from the links above, or through your package manager).

To install `ittyr`, run:

```sh
cargo install ittyr
```

## Known issues

Occasionally, `ittyr` will spit out meaningless strings of characters when started. This is simply caused by `minimodem` flipping the mark and space tones. Just restart `ittyr`. This only happens when the program first starts.