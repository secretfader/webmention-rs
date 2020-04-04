// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla Public
// License, version 2.0, which can be found in the included LICENSE
// file or at https://www.mozilla.org/en-US/MPL/2.0.

//! This module provides the utilities necessary to query a URL for endpoint
//! discovery, and sending Webmentions to detected endpoints via an `async`
//! HTTP client

use std::convert::{Into, TryFrom};
use url::Url;
use source::Source;
use crate::{parser, Error};

mod builder;
mod source;

/// Asynchronous implementation of a Webmention client with a simple Builder API
///
/// ## Example
/// ```rust
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
///     use webmention::Client;
///     let _ = Client::builder()
///         .source("https://webmention.rocks/test/1")
///         .target("https://webmention.rocks/update/1")
///         .target("https://webmention.rocks/update/2")
///         .build()?;
///     Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct Client {
    source: Url,
    targets: Option<Vec<Url>>,
}

impl Client {
    /// Create a new instance of the Webmention Client via Builder
    pub fn builder() -> builder::Builder {
        builder::Builder::new()
    }

    /// Create a new instance of the Webmention Client (with the Source URL
    /// preconfigured) via Builder
    pub fn source<T>(source: T) -> builder::Builder
    where
        Url: TryFrom<T>,
        <Url as TryFrom<T>>::Error: Into<Error>,
    {
        builder::Builder::new().source(source)
    }

    /// Run pre-configured operations, sending Webmention notifications if
    /// targets are configured and source discovery if not
    pub async fn send(self) -> parser::ParserResponse {
        let src = Source::new(self.source);
        parser::parse(src).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_client() {
        let c = Client::builder()
            .source("https://webmention.rocks")
            .target("https://webmention.rocks/update/1")
            .build()
            .unwrap();

        assert_eq!(
            c.source.to_string(),
            "https://webmention.rocks/".to_string()
        );
        assert_eq!(
            c.targets.unwrap()[0].to_string(),
            "https://webmention.rocks/update/1".to_string()
        );
    }

    #[test]
    fn build_client_with_query() {
        let c = Client::builder()
            .source("https://webmention.rocks?query=yes")
            .target("https://webmention.rocks/update/1?query=yes")
            .build()
            .unwrap();

        assert_eq!(
            c.source.to_string(),
            "https://webmention.rocks/?query=yes".to_string()
        );
        assert_eq!(
            c.targets.unwrap()[0].to_string(),
            "https://webmention.rocks/update/1?query=yes".to_string()
        );
    }

    #[test]
    fn build_invalid_client() {
        let c = Client::builder().build();
        assert_eq!(c.is_err(), true);
    }
}
