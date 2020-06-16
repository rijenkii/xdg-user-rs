![Maintenance](https://img.shields.io/maintenance/yes/2020)
[![crates.io](https://img.shields.io/crates/v/xdg-user)](https://crates.io/crates/xdg-user)
[![docs.rs](https://docs.rs/xdg-user/badge.svg)](https://docs.rs/xdg-user)

# xdg-user

This simple crate allows you to get paths to well known user directories,
using [`xdg-user-dirs`][1]s `user-dirs.dirs` file.

There are two ways of using this crate - with functions in the root of the
crate, or with the [`UserDirs`] struct. [`UserDirs`] will read and parse the
config file only once - when you call the [`UserDirs::new`] function.
Functions in the root will read and parse the config file EVERY TIME you
call them - so use them ONLY if you need to get one or two folders one or
two times.

## Example

```rust
println!("Pictures folder: {:?}", xdg_user::pictures()?);
println!("Music folder:    {:?}", xdg_user::music()?);

let dirs = xdg_user::UserDirs::new()?;
println!("Documents folder: {:?}", dirs.documents());
println!("Downloads folder: {:?}", dirs.downloads());
```

[1]: https://www.freedesktop.org/wiki/Software/xdg-user-dirs/

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
