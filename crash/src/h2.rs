
use enum_repr::EnumRepr;

/// This is sent by http2 clients after negotiating over ALPN, or when doing h2c.
pub const PREFACE: &[u8] = b"PRI * HTTP/2.0\r\n\r\nSM\r\n\r\n";

/// @see https://httpwg.org/specs/rfc9113.html#FrameTypes
#[EnumRepr(type = "u8")]
pub enum FrameType {
    Data = 0,
    Headers = 1,
    Priority = 2,
    RstStream = 3,
    Settings = 4,
    PushPromise = 5,
    Ping = 6,
    GoAway = 7,
    WindowUpdate = 8,
    Continuation = 9,
}

// @see https://httpwg.org/specs/rfc9113.html#FrameHeader
#[derive(Debug)]
pub struct Frame {
    pub frame_type: FrameType,
    pub flags: u8,
    pub reserved: u8,
    pub stream_id: u32,
    pub payload: OpaquePayload,
}