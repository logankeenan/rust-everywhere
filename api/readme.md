# API

Basic api for CRUD operations of a note.

## Tests

**Unit**
`cargo test`

**Acceptance**
`cargo build && . ./acceptance-test.sh`, note [jq](https://formulae.brew.sh/formula/jq) is required

## Release
* Depends on Docker and [cross](https://github.com/cross-rs/cross)
* `. ./build-release.sh`
