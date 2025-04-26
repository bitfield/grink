use anyhow::{Context, Result};
use regex::Regex;
use tokio::fs;

use std::{ffi::OsString, path::PathBuf};

pub struct UrlMatcher(Regex);

impl UrlMatcher {
    #[must_use]
    /// Constructs a new `UrlMatcher`.
    ///
    /// # Panics
    ///
    /// * If the regex pattern is invalid
    pub fn new() -> Self {
        Self(
            Regex::new(r"https?:\/\/[\w\d.:]+\/?[\w\d./?=#%:!\-]+")
                .expect("pattern should be valid"),
        )
    }

    /// Extracts all the URLs from `haystack`.
    #[must_use]
    pub fn urls<'h>(&self, haystack: &'h str) -> Vec<&'h str> {
        self.0.find_iter(haystack).map(|m| m.as_str()).collect()
    }
}

impl Default for UrlMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(PartialEq)]
pub enum Status {
    OK,
    Warning(String),
    Error(String),
    Skipped,
}

pub struct Link {
    pub url: String,
    pub status: Status,
    pub referrer: OsString,
}

/// # Errors
pub async fn scan(paths: &[PathBuf]) -> Result<Vec<Link>> {
    let matcher = UrlMatcher::new();
    let http = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36")
        .build()?;
    let mut links = Vec::new();
    for path in paths {
        let data = fs::read_to_string(path)
            .await
            .context(format!("reading {}", path.display()))?;
        for url in matcher.urls(&data) {
            match http.get(url).send().await {
                Err(e) => {
                    links.push(Link {
                        url: url.to_string(),
                        status: Status::Error(e.to_string()),
                        referrer: path.into(),
                    });
                }
                Ok(resp) => {
                    let status = if let Err(e) = resp.error_for_status() {
                        Status::Error(e.to_string())
                    } else {
                        Status::OK
                    };
                    links.push(Link {
                        url: url.to_string(),
                        status,
                        referrer: path.into(),
                    });
                }
            }
        }
    }
    Ok(links)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urls_correctly_extracts_valid_urls() {
        struct Case {
            input: &'static str,
            want: Vec<&'static str>,
        }
        let cases = vec![
            Case {
                input: "—Jerry Weinberg, [“Secrets of Consulting”](https://amzn.to/3uzcGE0)",
                want: vec![
                    "https://amzn.to/3uzcGE0"
                ],
            },
            Case {
                input: "([Listing hello/3](https://github.com/bitfield/tpg-tools2/blob/main/hello/3/cmd/hello/main.go))",
                want: vec![
                    "https://github.com/bitfield/tpg-tools2/blob/main/hello/3/cmd/hello/main.go"
                ]
            },
            Case {
                input:
                    "* [https://github.com/bitfield/gotestdox](https://github.com/bitfield/gotestdox)",
                want: vec![
                    "https://github.com/bitfield/gotestdox",
                    "https://github.com/bitfield/gotestdox",
                ],
            },
            Case {
                input:
                    "The standard library function [`slices.Equal`](https://pkg.go.dev/golang.org/x/exp/slices#Equal) can do this comparison for us, but there's an even better solution. There's a package called [`go-cmp`](https://github.com/google/go-cmp) that is really clever at comparing all kinds of Go data structures, and it's especially useful for tests.",
                want: vec![
                    "https://pkg.go.dev/golang.org/x/exp/slices#Equal",
                    "https://github.com/google/go-cmp",
                ],
            },
            Case {
                input:
                    " —[“Saturday Night Live”](https://www.youtube.com/watch?v=GmqeZl8OI2M)",
                want: vec![
                    "https://www.youtube.com/watch?v=GmqeZl8OI2M",
                ],
            },
            Case {
                input: "> —Andreas Klinger, [“Managing People”](https://klinger.io/posts/managing-people-%F0%9F%A4%AF)",
                want: vec![
                    "https://klinger.io/posts/managing-people-%F0%9F%A4%AF",
                ],
            },
            Case {
                input: "Test link: [local test server](http://127.0.0.1:63151/)",
                want: vec![
                    "http://127.0.0.1:63151/",
                ],
            },
            Case {
                input: "With all due respect to Descartes, he had it backwards. The fact that “I” am thinking indicates only that *there are thoughts*. The “I” who is supposedly in charge of them is a psychologically convenient fiction, no more. Or, as the poet Emily Dickinson put it, [“I'm nobody! Who are you?”](https://en.wikipedia.org/wiki/I%27m_Nobody!_Who_are_you%3F)",
                want: vec![
                    "https://en.wikipedia.org/wiki/I%27m_Nobody!_Who_are_you%3F",
                ],
            },
            Case {
                input: "—Stephen Hough, [“Problems Playing the Piano?”](https://web.archive.org/web/20210210210510/http://www.stephenhough.com/writings/selective/problems-playing-piano.php)",
                want: vec![
                    "https://web.archive.org/web/20210210210510/http://www.stephenhough.com/writings/selective/problems-playing-piano.php",
                ],
            },
        ];
        let matcher = UrlMatcher::new();
        for case in cases {
            let got = matcher.urls(case.input);
            assert_eq!(case.want, got, "{}", case.input);
        }
    }
}
