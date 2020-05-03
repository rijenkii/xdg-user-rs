# xdg-user

This simple crate allows you to get paths to "well known" user directories,
using [`xdg-user-dirs`][1]s `user-dirs.dirs` file.

## Example

```rust
let dirs = xdg_user::UserDirs::new()?;
println!("Documents folder: {:?}", dirs.documents());
println!("Downloads folder: {:?}", dirs.downloads());
```

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

[1]: https://www.freedesktop.org/wiki/Software/xdg-user-dirs/