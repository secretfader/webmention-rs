// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla
// Public License, version 2.0, which can be found in the
// LICENSE file or at https://www.mozilla.org/en-US/MPL/2.0/.

/// Convenience type for `std::result::Result` for operations
/// within the Webmention crate
pub type Result<T> = std::result::Result<T, Error>;

/// Convenience type for errors that may be produced within
/// the Webmention crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "client")]
    #[error("Invalid URI {0}")]
    InvalidUri(http::uri::InvalidUri),
}

#[cfg(feature = "client")]
impl std::convert::From<http::uri::InvalidUri> for Error {
    fn from(e: http::uri::InvalidUri) -> Self {
        Error::InvalidUri(e)
    }
}
