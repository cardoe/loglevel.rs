# ChangeLog

## 0.4.0

### Changed

- Now depends on log 0.4.1.

## 0.3.1

### Added

- Timestamp now implements FromStr to make arg parsers quicker to implement
- Added a StructOpt example to the crate documentation and examples

### Changed

- Improved clap example and added Timestamp support

## 0.3.0

### Added

- a large example under examples to detail how module level logging
  works

### Changed

- bump minimum Rust version to 1.16.0
- allow all log 0.3.x releases
- fix situations where including `a::b` also included `a::baz`

## 0.2.4

### Changed

- pinned log to 0.3.8 or older to retain Rust 1.13.0 support

## 0.2.3

### Added

- clap example
- support timestamps with microsecond and nanosecond granularity

### Changed

- improved performance (https://github.com/cardoe/stderrlog-rs/pull/2)
- improved docopt example in README
- migrated from time to chrono

### Thanks

- Thanks to Zachary Dremann <dremann@gmail.com> for the performance
  improvements and the nudge to improve the docs.
