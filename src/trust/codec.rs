use bytes::{BufMut, BytesMut};
use std::io;
use tokio_util::codec::{Decoder, Encoder};

pub struct TrustTcpCodec;

impl Decoder for TrustTcpCodec {
    type Item = String;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if let Some(nl_index) = find_subsequence(src, b"<NL>") {
            let line = src.split_to(nl_index + 4);
            return Ok(Some(
                String::from_utf8_lossy(&line[..nl_index])
                    .trim()
                    .to_string(),
            ));
        }

        Ok(None)
    }
}

impl Encoder<String> for TrustTcpCodec {
    type Error = io::Error;

    fn encode(&mut self, msg: String, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg_ref: &[u8] = msg.as_ref();
        dst.put(msg_ref);

        Ok(())
    }
}

fn find_subsequence<T>(haystack: &[T], needle: &[T]) -> Option<usize>
where
    for<'a> &'a [T]: PartialEq,
{
    haystack
        .windows(needle.len())
        .position(|window| window == needle)
}
