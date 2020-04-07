// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla
// Public License, version 2.0, which can be found in the
// LICENSE file or at https://www.mozilla.org/en-US/MPL/2.0/.A

//! This module provides utilities to scrape and process HTML,
//! returning tags that are relevant to Webmention, if available

use crate::Result;
use scraper::Html;
use url::Url;

/// Download markup from a requested URL and scan for Webmention
/// endpoints
pub async fn query(url: Url) -> Result<()> {
    let req = reqwest::get(url).await?.text().await?;
    Ok(())
}
