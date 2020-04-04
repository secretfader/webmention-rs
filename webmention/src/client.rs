// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla
// Public License, version 2.0, which can be found in the
// LICENSE file or at https://www.mozilla.org/en-US/MPL/2.0/.A

use crate::result::Result;
use http::Uri;
use std::convert::{Into, TryFrom};

/// Asynchronous implementation of a Webmention client with
/// an easy-to-use builder.
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
pub struct Client {
    source: Uri,
    targets: Option<Vec<Uri>>,
}

impl Client {
    /// Create a new instance of the Webmention Client via Builder
    pub fn builder() -> Builder {
        Builder::new()
    }

    /// Create a new instance of the Webmention Client (with the
    /// Source URI pre-configured) via Builder
    pub fn source<T>(source: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,
    {
        Builder::new().source(source)
    }

    /// Run pre-configured operations, sending Webmention notifications
    /// if targets are configured, and source discovery if not.
    pub async fn run(self) -> Result<()> {
         match self.mode() {
            Mode::Send => Ok(self.send().await?),
            Mode::Query => Ok(self.query().await?),
        }
    }

    fn mode(&self) -> Mode {
        match &self.targets {
            Some(_) => Mode::Send,
            None => Mode::Query,
        }
    }

    async fn send(&self) -> Result<()> {
        Ok(())
    }

    async fn query(&self) -> Result<()> {
        Ok(())
    }
}

impl Default for Client {
    #[inline]
    fn default() -> Self {
        Client {
            source: Uri::default(),
            targets: None,
        }
    }
}

enum Mode {
    Send,
    Query,
}

/// Builder to ease creating new Webmention Clients
pub struct Builder {
    inner: Result<Client>,
}

impl Builder {
    /// Create a new Builder instance, which is used to construct
    /// a Webmention Client
    fn new() -> Self {
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
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut client| {
            client.source = TryFrom::try_from(source).map_err(Into::into)?;
            Ok(client)
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
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut client| {
            let value = TryFrom::try_from(target).map_err(Into::into)?;

            if let Some(targets) = &mut client.targets {
                targets.push(value);
            } else {
                client.targets = Some(vec![value]);
            }

            Ok(client)
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
    pub fn build(self) -> Result<Client> {
        self.inner.map(move |client| Client { ..client })
    }

    // Private implementation to simplify the Builder
    fn and_then<F>(self, f: F) -> Self
    where
        F: FnOnce(Client) -> Result<Client>,
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
            inner: Ok(Client::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_client() {
        let c = Client::builder()
            .source("https://www.secretfader.com/")
            .target("https://www.secretfader.com/2")
            .build()
            .unwrap();

        assert_eq!(
            c.source.to_string(),
            "https://www.secretfader.com/".to_string()
        );
        assert_eq!(
            c.targets.unwrap()[0].to_string(),
            "https://www.secretfader.com/2".to_string()
        );
    }
}
