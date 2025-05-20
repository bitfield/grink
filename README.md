[![Crate](https://img.shields.io/crates/v/grink.svg)](https://crates.io/crates/grink)
[![Docs](https://docs.rs/grink/badge.svg)](https://docs.rs/grink)
![CI](https://github.com/bitfield/grink/actions/workflows/ci.yml/badge.svg)
![Audit](https://github.com/bitfield/grink/actions/workflows/audit.yml/badge.svg)
![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# grink

Grink is a tool to find and check web links in text files (for example, Markdown source files). It “greps links”, if you will.

### Installation

```sh
cargo install grink
```

### Usage

Grink reads all the files you specify, extracts anything that looks like a URL, and checks if it returns an OK status, reporting the error otherwise.

```sh
grink book/*.md
```

```txt
[ERROR] (HTTP status client error (404 Not Found) for url (https://example.com/bogus)) https://example.com/bogus - referrer: book/chapter1.md
Links: 8 (7 OK, 1 errors, 0 warnings)
```
