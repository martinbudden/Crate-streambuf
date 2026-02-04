#![no_std]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(unused_must_use)]

mod stream_buf_reader;
mod stream_buf_writer;

pub use stream_buf_reader::StreamBufReader;
pub use stream_buf_writer::StreamBufWriter;
