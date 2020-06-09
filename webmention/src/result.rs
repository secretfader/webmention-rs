// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla Public
// License, version 2.0, which can be found in the included LICENSE
// file or at https://www.mozilla.org/en-US/MPL/2.0.

use std::convert::From;

/// Convenience alias `std::result::Result` for operations that may
/// fail within the Webmention crate
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Convenience type for errors that may be produced from within the Webmention
/// crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[cfg(feature = "client")]
    #[error("Client {0}")]
    /// Problem occurred while initializing or executing the Webmention Client
    Client(ClientError),

    #[error("Parse {0}")]
    /// Problem occurred while attempting to parse or validate URLs
    Parse(ParseError),
}

/// Errors that may be produced by the Webmention Parser
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid URL {0}")]
    /// Failed to parse a valid URL
    InvalidUrl(url::ParseError),

    #[error("Invalid URI {0}")]
    /// Failed to parse a valid URI
    InvalidUri(http::uri::InvalidUri),

    #[error("Invalid HTTP header")]
    /// Malformatted HTTP `Link` header
    InvalidHeader,

    #[error("Invalid value")]
    /// Malformatted Webmention key value
    InvalidValue,
}

#[cfg(feature = "client")]
/// Errors that may be produced by the Webmention Client
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[cfg(feature = "client")]
    #[error("Invalid Parameter {0}")]
    /// Problem occurred while initializing the Client
    InvalidParam(&'static str),

    #[cfg(feature = "client")]
    #[error("HTTP Request {0}")]
    /// Problem occurred during execution of an HTTP request
    Request(reqwest::Error),
}

#[cfg(feature = "client")]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Client(ClientError::Request(e))
    }
}

#[cfg(feature = "client")]
impl From<ClientError> for Error {
    fn from(e: ClientError) -> Self {
        Error::Client(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError)-> Self {
        Error::Parse(e)
    }
}

impl From<url::ParseError> for Error {
    fn from(e: url::ParseError) -> Self {
        Error::Parse(ParseError::InvalidUrl(e))
    }
}

impl From<http::uri::InvalidUri> for Error {
    fn from(e: http::uri::InvalidUri) -> Self {
        Error::Parse(ParseError::InvalidUri(e))
    }
}
