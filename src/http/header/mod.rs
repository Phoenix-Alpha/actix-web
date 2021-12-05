//! A Collection of Header implementations for common HTTP Headers.
//!
//! ## Mime Types
//! Several header fields use MIME values for their contents. Keeping with the strongly-typed theme,
//! the [mime] crate is used in such headers as [`ContentType`] and [`Accept`].

use std::fmt;

use bytes::{Bytes, BytesMut};

// re-export from actix-http
// - header name / value types
// - relevant traits for converting to header name / value
// - all const header names
// - header map
// - the few typed headers from actix-http
// - header parsing utils
pub use actix_http::http::header::*;

mod accept;
mod accept_charset;
mod accept_encoding;
mod accept_language;
mod allow;
mod cache_control;
mod content_disposition;
mod content_language;
mod content_range;
mod content_type;
mod date;
mod encoding;
mod entity;
mod etag;
mod expires;
mod if_match;
mod if_modified_since;
mod if_none_match;
mod if_range;
mod if_unmodified_since;
mod last_modified;
mod macros;
mod preference;
mod range;

#[cfg(test)]
pub(crate) use macros::common_header_test;
pub(crate) use macros::{common_header, common_header_test_module};

pub use self::accept::Accept;
pub use self::accept_charset::AcceptCharset;
pub use self::accept_encoding::AcceptEncoding;
pub use self::accept_language::AcceptLanguage;
pub use self::allow::Allow;
pub use self::cache_control::{CacheControl, CacheDirective};
pub use self::content_disposition::{ContentDisposition, DispositionParam, DispositionType};
pub use self::content_language::ContentLanguage;
pub use self::content_range::{ContentRange, ContentRangeSpec};
pub use self::content_type::ContentType;
pub use self::date::Date;
pub use self::encoding::Encoding;
pub use self::entity::EntityTag;
pub use self::etag::ETag;
pub use self::expires::Expires;
pub use self::if_match::IfMatch;
pub use self::if_modified_since::IfModifiedSince;
pub use self::if_none_match::IfNoneMatch;
pub use self::if_range::IfRange;
pub use self::if_unmodified_since::IfUnmodifiedSince;
pub use self::last_modified::LastModified;
pub use self::preference::Preference;
pub use self::range::{ByteRangeSpec, Range};

/// Format writer ([`fmt::Write`]) for a [`BytesMut`].
#[derive(Debug, Default)]
struct Writer {
    buf: BytesMut,
}

impl Writer {
    /// Constructs new bytes writer.
    pub fn new() -> Writer {
        Writer::default()
    }

    /// Splits bytes out of writer, leaving writer buffer empty.
    pub fn take(&mut self) -> Bytes {
        self.buf.split().freeze()
    }
}

impl fmt::Write for Writer {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.buf.extend_from_slice(s.as_bytes());
        Ok(())
    }

    #[inline]
    fn write_fmt(&mut self, args: fmt::Arguments<'_>) -> fmt::Result {
        fmt::write(self, args)
    }
}
