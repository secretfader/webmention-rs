// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla Public License,
// version 2.0, which can be found in the included LICENSE file or at
// https://www.mozilla.org/en-US/MPL/2.0.

//! Types common to Webmention requests, responses, and link feeds.

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
use serde::{Serialize, Deserialize};
use url::Url;

/// Types of events that can be sent or received as Webmentions.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MentionType {
    /// Indicator that the event in reply to a previous entry
    Reply,
    /// Indicator that the event liked a previous entry
    Like,
    /// Indicator that the sender "reposted" a previous entry
    Repost,
    /// Indicator that the sender "bookmarked" a previous entry
    Bookmark,
    /// Indicator that the sender accepted or rejected an invitation
    Rsvp,
    /// Wrapper for other mention types that aren't yet supported
    Mention,
}

/// The Author (or publisher) of a Webmention event.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Author {
    /// Name of the Author
    name: String,
    /// URL of the Author's homepage
    url: String,
}

/// Primary type for representing Webmention events. Includes the Author, title,
/// content, source URL, and type of event.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mention {
    author: Author,
    title: String,
    content: String,
    url: Url,
    mention_type: MentionType,
}
