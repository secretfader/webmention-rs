// Copyright 2020 Nicholas Young. All rights reserved.
//
// Public License, version 2.0, which can be found in the
// LICENSE file or at https://www.mozilla.org/en-US/MPL/2.0/.

//! Types common to Webmention requests, responses, and link feeds.

use url::Url;

/// Events that can be sent and received as Webmentions
#[derive(Debug)]
pub enum MentionType {
    /// Indicator that the sender is "replying" to a previous entry
    Reply,
    /// Indicator that the sender "liked" a previous entry
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

/// Author who created the content and sent any associated payload
#[derive(Debug)]
pub struct Author {
    /// Name of the Author
    pub name: String,
    /// URL of the Author's homepage
    pub url: String,
}

/// Representation of a Webmention payload
#[derive(Debug)]
pub struct Mention {
    /// Author of a Webmention (and its content)
    pub author: Author,
    /// XYZ
    pub title: String,

    /// XYZ
    pub content: String,

    /// XYZ
    pub url: Url,

    /// XYZ
    pub mention_type: MentionType,
}
