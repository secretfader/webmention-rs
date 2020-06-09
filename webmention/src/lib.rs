// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla Public License,
// version 2.0, which can be found in the included LICENSE file or at
// https://www.mozilla.org/en-US/MPL/2.0.

#![deny(missing_docs, missing_debug_implementations)]
#![cfg_attr(test, deny(rust_2018_idioms, warnings))]

//! Utilities to send and retrieve Webmentions, an open notification standard
//! that is part of the IndieWeb ecosystem.
//!
//! This crate contains types that can be used to handle Webmentions in other
//! applications. Optionally, it includes a Client which can be used to perform
//! endpoint discovery and send Webmentions to a list of URLs.

#[cfg(feature = "client")]
mod client;
pub mod parser;
mod result;

#[cfg(feature = "client")]
pub use crate::client::Client;
pub use crate::result::{Error, Result};

/// Re-export of [`async_trait`](https://docs.rs/async-trait) which is guaranteed
/// to work. This helps with managing library versions for other types that may
/// want to implement `Parsable`.
pub use async_trait::async_trait;
