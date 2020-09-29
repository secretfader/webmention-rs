// Copyright 2020 Nicholas Young. All rights reserved.
//
// Use of this source code file is governed by the Mozilla Public License,
// version 2.0, which can be found in the included LICENSE file or at
// https://www.mozilla.org/en-US/MPL/2.0.

//! Facilities to parse Webmention metadata from HTTP `Link` headers and HTML
//! `<link>` tags, including both relative and absolute URIs.
//!
//! Items of note from the Webmention specification and the "Webmention Rocks"
//! test suite:
//!
//! * URIs, which may happen to be relative references, are valid within the
//!   context of HTTP `Link` headers.
//!
//! * Since the `Client` must be able to resolve URLs returned by the parser,
//!   final output must be in the form of *valid* URLs. If a URL cannot be built
//!   successfully from provided components it will be discarded.

use crate::result::{ParseError, Result};
use http::Uri;
use regex::Regex;
use scraper::{Html, Selector};
use std::convert::TryFrom;
use url::Url;

/// Provide a single entrypoint for this module, parsing HTTP header values if
/// available, and HTML link tags if not.
pub async fn parse<T: Parsable>(payload: T) -> ParserResponse {
    let (url, (headers, body)) = payload.into_parser_parts().await?;

    if let Some(found) = parse_link_headers(headers)? {
        return Refkind::from_parts(url.clone(), found)
            .map_err(|_| ParseError::InvalidValue)?
            .into_response();
    }

    if let Some(found) = parse_link_tags(body)? {
        return Refkind::from_parts(url.clone(), found)
            .map_err(|_| ParseError::InvalidValue)?
            .into_response();
    };

    Ok(None)
}

/// Convenience type for Webmention parser responses, which allows a simple
/// default return type of `Ok(None)`, while providing tidy error handling
/// should anything go wrong.
pub type ParserResponse = Result<Option<Url>>;

/// Trait implemented by types that wish to be handled by the Webmention parser.
#[crate::async_trait]
pub trait Parsable {
    /// Extract the base URL, plus a tuple of HTTP `Link` headers and the body
    /// payload.
    async fn into_parser_parts(self) -> Result<(url::Url, (Vec<String>, String))>;
}

// Evaluate HTTP `Link` headers for Webmention support and return the first
// detected endpoint.
fn parse_link_headers(value: Vec<String>) -> Result<Option<String>> {
    let mut res: Option<String> = None;

    for v in value {
        if let Some(v) = parse_link_header(v)? {
            res = Some(v.to_string());
        }
    }

    if let Some(res) = res {
        return Ok(Some(res));
    }

    Ok(None)
}

// Evaluate a single HTTP `Link` header for Webmention support, returning a
// value if found, none if not, and erroring only if a parser error occurs.
fn parse_link_header(v: String) -> Result<Option<String>> {
    let mut res: Option<String> = None;

    let exp = Regex::new(r#"[<>"\s]"#).unwrap();
    let re = exp.replace_all(&v, "");
    let links = re.split(',');

    for link in links {
        // Exit the loop if a valid tag has already been found
        if res.is_some() {
            break;
        }

        let payload: Vec<&str> = link.split(';').collect();

        let uri: Option<&str> = if payload.len() >= 2 {
            Some(payload.as_slice()[0])
        } else {
            None
        };

        let rel: Vec<&str> = payload[1].split('=').collect();
        if rel.len() < 2 {
            break;
        }

        let pattern = &rel[1].to_lowercase().find("webmention");

        match (&rel[0], pattern) {
            (&"rel", Some(_)) => {
                if let Some(v) = uri {
                    res = Some(v.to_string());
                }
            }
            _ => {}
        }
    }

    Ok(res)
}

// Evaluate HTML source for Webmention support and return the first detected
// endpoint.
fn parse_link_tags(value: String) -> Result<Option<String>> {
    let tree = Html::parse_document(&value);
    let iselect = Selector::parse(r#"[rel*=webmention]"#).unwrap();
    let mut res: Option<&str> = None;

    for el in tree.select(&iselect) {
        // Exit the loop if a valid tag has already been found
        if res.is_some() {
            break;
        }

        match (el.value().attr("rel"), el.value().attr("href")) {
            (Some("webmention"), Some(v)) => {
                res = Some(v);
            }
            _ => {}
        };
    }

    if let Some(res) = res {
        return Ok(Some(res.to_string()));
    }

    Ok(None)
}

#[derive(Debug)]
enum Refkind {
    Abs(Url),
    Rel(Uri),
    Unknown((Url, String)),
}

impl Refkind {
    /// Attempt to construct an instance of `RefKind` from a base URL and
    /// trailing components.
    fn from_parts(src: Url, v: &str) -> Result<Self, ParseRefError> {
        if v.is_empty() {
            return Err(ParseRefError::InvalidInput);
        }

        if let Ok(url) = Url::try_from(v) {
            return Ok(Self::Abs(url));
        }

        if let Ok(uri) = Uri::try_from(v) {
            return Ok(Self::Rel(uri));
        }

        Ok(Self::Unknown((src, v.to_string())))
    }

    // Attempt to convert an instance of `RefKind` to `ParserResponse`.
    fn into_response(&self) -> ParserResponse {
        let val = match &self {
            Self::Abs(u) => return Ok(Some(u.clone())),
            Self::Rel(u) => Some((u,())),
            Self::Unknown((src, val)) => Some((src, val)),
        };

        // Attempt to build a URL out of the fewest valid components
        if let Some(v) = match (val.scheme(), val.authority(), val.path_and_query()) {
            (Some(s), Some(a), None) => Some(format!("{}://{}", s, a)),
            (Some(s), Some(a), Some(p)) => Some(format!("{}://{}{}", s, a, p)),
            _ => None,
        } {
            log::debug!("Attempting to construct URL from: {}", &v);
            return Ok(Some(Url::parse(&v)?));
        }

        // Next idea: how about the existing path and query joined with a base URL?
        if let Some(v) = match val.path_and_query() {
            Some(path) => match val.origin() {
                url::Origin::Tuple(s, h, p) => match p {
                    80 | 443 => Some(format!("{}://{}{}", s, h, path)),
                    _ => Some(format!("{}://{}:{}{}", s, h, p, path)),
                },
                _ => None,
            },
            _ => None,
        } {
            log::debug!("Attempting to construct URL from: {}", &v);
            return Ok(Some(Url::parse(&v)?));
        }

        Err(ParseError::InvalidHeader)?
    }
}

#[derive(Debug)]
enum ParseRefError {
    InvalidInput,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn refkind_to_string(r: Refkind) -> String {
        match r {
            Refkind::Abs(u) => u.to_string(),
            Refkind::Rel(u) => u.to_string(),
        }
    }

    #[test]
    fn refkind_from_valid_url() {
        let src = Url::parse("https://webmention.rocks").unwrap();
        let rel = Refkind::from_str("https://webmention.rocks").unwrap();
        let inner = refkind_to_string(rel);

        assert_eq!(inner, src.to_string());
    }

    #[test]
    fn refkind_from_invalid_url() {
        let src = http::Uri::from_str("/test/1?query=true").unwrap();
        let rel = Refkind::from_str("/test/1?query=true").unwrap();
        let inner = refkind_to_string(rel);

        assert_eq!(inner, src.to_string());
    }

    #[test]
    fn refkind_from_invalid_ref() {
        let rel = Refkind::from_str("");

        assert!(rel.is_err());
    }
}
