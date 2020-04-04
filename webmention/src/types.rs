// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla
// Public License, version 2.0, which can be found in the
// LICENSE file or at https://www.mozilla.org/en-US/MPL/2.0/.A

use url::Url;

/// Events that can be sent and received as Webmentions
#[derive(Debug)]
pub enum MentionType {
    Reply,
    Like,
    Repost,
    Bookmark,
    Rsvp,
    Mention,
}

/// Author who created the content and sent the associated
/// Webmention payload
#[derive(Debug)]
pub struct Author {
    pub name: String,
    pub url: String,
}

/// Representation of a Webmention payload
#[derive(Debug)]
pub struct Mention {
    pub author: Author,
    pub title: String,
    pub content: String,
    pub url: Url,
    pub mention_type: MentionType,
}
