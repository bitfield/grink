use regex::Regex;

pub struct UrlMatcher(Regex);

impl UrlMatcher {
    #[must_use]
    /// Constructs a new `UrlMatcher`.
    /// 
    /// # Panics
    /// 
    /// * If the regex pattern is invalid
    pub fn new() -> Self {
        Self(Regex::new(r"https?:\/\/[\w\d.:]+\/?[\w\d./?=#%\-]+").expect("pattern should be valid"))
    }

    /// Extracts all the URLs from `haystack`.
    #[must_use]
    pub fn urls<'h>(&self, haystack: &'h str) -> Vec<&'h str> {
        self.0.find_iter(haystack)
            .map(|m| m.as_str())
            .collect()
    }
}

impl Default for UrlMatcher {
    fn default() -> Self {
        Self::new()
    }
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
        ];
        let matcher = UrlMatcher::new();
        for case in cases {
            let got = matcher.urls(case.input);
            assert_eq!(case.want, got, "{}", case.input);
        }
    }
}
