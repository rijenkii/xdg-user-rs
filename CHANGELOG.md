# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1] - 2020-05-14
### Changed
- Use `home::home_dir` instead of a stolen function

## [0.2.0] - 2020-05-04
### Added
- Root functions for getting just one folder path
- Add support for `cargo readme`
### Changed
- Move utility functions in `util` module

## [0.1.2] - 2020-05-04
### Changed
- Swap regex-based parser for a hand-written one

## [0.1.1] - 2020-05-04
### Fixed
- Don't panic when config file does not exists
- Don't panic on config file read errors
- Don't panic if config file is not utf8

[Unreleased]: https://github.com/rijenkii/xdg-user-rs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/rijenkii/xdg-user-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/rijenkii/xdg-user-rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/rijenkii/xdg-user-rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/rijenkii/xdg-user-rs/compare/v0.1.0...v0.1.1
