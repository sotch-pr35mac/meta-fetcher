//! # Metadata Fetcher
//! Metadata Fetcher is a utility for grabbing website metadata; useful for tasks like generating
//! link previews. Its built ontop of [ureq](https://crates.io/crates/ureq).
//!
//! ### Behavior
//! Metadata Fetcher first looks for a site's Open Graph Protocol (OGP) metadata and if not found
//! looks for the standard HTML metadata. If not metadata is found, it returns `None` for the
//! missing field. This module also respects a site's `robots.txt` file.
//!
//! ### Usage
//! ```rust
//! use meta_fetcher::fetch_metadata;
//!
//! // Grab the metadata for some URL
//! let meta = fetch_metadata("http://example.com").unwrap();
//!
//! assert_eq!(meta.title, Some("Example Domain".to_string()));
//! ```
use select::{
    document::Document,
    predicate::{Attr, Name},
};
use texting_robots::{get_robots_url, Robot};

// User agent string to be used in HTTP requests.
const USER_AGENT: &str = "MetaFetcher/1.0";

/// This struct represents the metadata of a webpage.
/// It contains the title, description, and image of the webpage, if available.
/// Populated first by the Open Graph Protocol (OGP) metadata and then by the standard HTML metadata.
/// If no metadata is found, the field is `None`.
#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image: Option<String>,
}

impl Metadata {
    /// Create a new `Metadata` struct with the given title, description, and image.
    pub fn new(title: Option<String>, description: Option<String>, image: Option<String>) -> Self {
        Self {
            title,
            description,
            image,
        }
    }

    /// Get the metadata from the given URL. Please be aware, this function lacks the robots.txt check.
    /// ```rust
    /// use meta_fetcher::Metadata;
    ///
    /// let meta = Metadata::from_url("http://example.com").unwrap();
    /// assert_eq!(meta.title, Some("Example Domain".to_string()));
    /// ```
    pub fn from_url(url: &str) -> anyhow::Result<Self> {
        let site = ureq::get(url)
            .set("User-Agent", USER_AGENT)
            .call()?
            .into_string()?;

        let document = Document::from(site.as_str());
        let title: Option<String> = document
            .find(Attr("property", "og:title"))
            .next()
            .and_then(|n| n.attr("content"))
            .map(|t| t.to_string())
            .or_else(|| document.find(Name("title")).next().map(|t| t.text()));
        let description: Option<String> = document
            .find(Attr("property", "og:description"))
            .next()
            .and_then(|n| n.attr("context"))
            .map(|t| t.to_string())
            .or_else(|| {
                document
                    .find(Attr("name", "description"))
                    .next()
                    .and_then(|n| n.attr("content"))
                    .map(|t| t.to_string())
            });
        let image: Option<String> = document
            .find(Attr("property", "og:image"))
            .next()
            .and_then(|n| n.attr("content"))
            .map(|t| t.to_string());

        Ok(Self {
            title,
            description,
            image,
        })
    }
}

/// Check if the given URL is allowed to be crawled by the user agent specified in the `USER_AGENT` constant.
fn robots_allowed(url: &str) -> anyhow::Result<bool> {
    let robots_url = get_robots_url(url)?;
    let robots_response = ureq::get(&robots_url).set("User-Agent", USER_AGENT).call();
    match robots_response {
        Ok(response) => {
            let robots_txt = response.into_string()?;
            let robots = Robot::new(USER_AGENT, robots_txt.as_bytes())?;

            Ok(robots.allowed(url))
        }
        Err(_) => Ok(true),
    }
}

/// Fetch the metadata from the given URL.
/// ```rust
/// use meta_fetcher::fetch_metadata;
///
/// let meta = fetch_metadata("http://example.com").unwrap();
///
/// assert_eq!(meta.title, Some("Example Domain".to_string()));
/// assert_eq!(meta.description, None);
/// assert_eq!(meta.image, None);
/// ```
pub fn fetch_metadata(url: &str) -> anyhow::Result<Metadata> {
    if robots_allowed(url)? {
        Metadata::from_url(url)
    } else {
        Err(anyhow::anyhow!(
            "Not allowed to crawl page as specified in site's robots.txt file."
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_url() {
        let result = fetch_metadata("hi there");
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_url() {
        let result = fetch_metadata("");
        assert!(result.is_err());
    }

    #[cfg(feature = "network-tests")]
    #[test]
    fn test_no_robots() {
        // This test depends on data from a website remaining the same as when this test was written
        // which is out of our control. Not the best idea here, but I'm short on time. If these
        // tests fail because data from the source has changed, let's just udpate the expected
        // conditions.
        let expected = Metadata::new(Some("Example Domain".to_string()), None, None);
        let actual = fetch_metadata("http://example.com").unwrap();
        assert_eq!(expected, actual);
    }

    #[cfg(feature = "network-tests")]
    #[test]
    fn test_fetch_metadata() {
        // This test depends on data from a website remaining the same as when this test was written
        // which is out of our control. Not the best idea here, but I'm short on time. If these
        // tests fail because data from the source has changed, let's just udpate the expected
        // conditions.
        let expected = Metadata::new(
            Some("What are The 5 Love Languages?".to_string()), 
            Some("Learn the 5 Love Languages® and discover how it all started.".to_string()), 
            Some(
                "https://5lovelanguages.com/img/8af021a4-77a8-4984-8b6a-1924c93f8b2f/og_learn.jpg?fm=jpg&q=80&fit=max&crop=1200%2C627%2C0%2C0".to_string()
            )
        );
        let actual = fetch_metadata("https://5lovelanguages.com/learn").unwrap();
        assert_eq!(expected, actual);
    }

    #[cfg(feature = "network-tests")]
    #[test]
    fn test_metadata_from_url() {
        // This test depends on data from a website remaining the same as when this test was written
        // which is out of our control. Not the best idea here, but I'm short on time. If these
        // tests fail because data from the source has changed, let's just udpate the expected
        // conditions.
        let expected = Metadata::new(Some("Example Domain".to_string()), None, None);
        let actual = Metadata::from_url("http://example.com").unwrap();
        assert_eq!(expected, actual);
    }
}
