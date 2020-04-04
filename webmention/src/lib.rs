// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla
// Public License, version 2.0, which can be found in the
// LICENSE file or at https://www.mozilla.org/en-US/MPL/2.0/.

//! Utilities to send and retrieve Webmentions, an open
//! notification standard that is part of the IndieWeb
//! ecosystem.
//!
//! This crate contains types that can be used to parse
//! Webmentions in other applications. Optionally, it
//! also includes a Client which can be used to crawl a
//! given URL for mentioned URLs, perform endpoint
//! discovery and send Webmentions to a list of URLs.

#[cfg(feature = "client")]
mod client;
mod result;
pub mod types;

pub use crate::result::{Result, Error};
#[cfg(feature = "client")]
pub use crate::client::Client;
