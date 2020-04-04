// Copyright 2020 Nichoas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla Public License,
// version 2.0, which can be found in the included LICENSE file or at
// https://www.mozilla.org/en-us/MPL/2.0.

use crate::result::{ClientError, Error, Result};
use super::Client;
use std::convert::TryFrom;
use url::Url;

/// Present a simplified, validation-capable API for building instances of the
/// Webmention Client
#[derive(Debug)]
pub struct Builder {
    inner: Result<Options>,
}

/// Options that may be selected during Client construction
#[derive(Debug, Default)]
pub struct Options {
    source: Option<Url>,
    targets: Option<Vec<Url>>,
}

impl Builder {
    /// Create a new Builder, which will then be used used to configure a new
    /// Webmention Client
    pub fn new() -> Self {
        Builder::default()
    }

    /// Set the source URL for a Webmention Client
    ///
    /// ## Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     use webmention::Client;
    ///
    ///     let _ = Client::builder()
    ///         .source("https://webmention.rocks/test/1")
    ///         .build()?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    pub fn source<T>(self, source: T) -> Self
    where
        Url: TryFrom<T>,
        <Url as TryFrom<T>>::Error: Into<Error>,
    {
        self.and_then(move |mut opts| {
            opts.source = Some(TryFrom::try_from(source).map_err(Into::into)?);
            Ok(opts)
        })
    }

    /// Append target URLs to a Webmention Client
    ///
    /// ## Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     use webmention::Client;
    ///
    ///     let _ = Client::builder()
    ///         .source("https://webmention.rocks/test/1")
    ///         .target("https://webmention.rocks/update/1")
    ///         .build()?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    pub fn target<T>(self, target: T) -> Self
    where
        Url: TryFrom<T>,
        <Url as TryFrom<T>>::Error: Into<Error>,
    {
        self.and_then(move |mut opts| {
            let value = TryFrom::try_from(target).map_err(Into::into)?;

            if let Some(targets) = &mut opts.targets {
                targets.push(value);
            } else {
                opts.targets = Some(vec![value]);
            }

            Ok(opts)
        })
    }

    /// Validate parameters and instantiate a new Webmention Client
    ///
    /// ## Examples
    ///
    /// ```
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     use webmention::Client;
    ///
    ///     let _ = Client::builder()
    ///         .source("https://webmention.rocks/test/1")
    ///         .target("https://webmention.rocks/update/1")
    ///         .build()?;
    ///
    ///     Ok(())
    /// # }
    /// ```
    pub fn build(self) -> Result<super::Client> {
        self.inner
            .and_then(move |opts| match opts.source {
                Some(_) => Ok(opts),
                None => Err(ClientError::InvalidParam("Source is required"))?,
            })
            .map(move |opts| Client {
                source: opts.source.unwrap(),
                targets: opts.targets,
            })
    }

    // Private implementation to simplify the Builder
    fn and_then<F>(self, f: F) -> Self
    where
        F: FnOnce(Options) -> Result<Options>,
    {
        Builder {
            inner: self.inner.and_then(f),
        }
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Self {
        Builder {
            inner: Ok(Options::default()),
        }
    }
}
