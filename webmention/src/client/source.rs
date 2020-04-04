// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla Public
// License, version 2.0, which can be found in the included LICENSE
// file or at https://www.mozilla.org/en-US/MPL/2.0.

//! Provides a default implementation of the Parsable trait, which specifies
//! types that can be handled by the Webmention parser.

use crate::{parser::Parsable, result::Result};

/// Decouple our HTTP client types that can be parsed.
pub struct Source(url::Url);

impl Source {
    /// Construct a new instance of Source from a provided URL.
    pub fn new(url: url::Url) -> Self {
        Source(url)
    }
}

#[async_trait::async_trait]
impl Parsable for Source {
    async fn into_parser_parts(self) -> Result<(url::Url, (Vec<String>, String))> {
        let req = reqwest::get(self.0.clone()).await?;

        let links = req.headers() 
            .get_all("link")
            .iter()
            .map(|v| match v.to_str() {
                Ok(v) => Ok(v.to_string()),
                Err(e) => Err(e)
            })
            .filter_map(std::result::Result::ok)
            .collect();

        let body = req.text().await?;

        Ok((self.0, (links, body)))
    }
}
