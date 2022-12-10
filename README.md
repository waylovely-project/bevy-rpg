
<div align="center">

# 🌷🌷 Bevy x RPG Dialog 🌸🌸
*RPG Dialog support for Bevy, written in pure Rust!*

</div>

`bevy-rpg` is a plugin crate that adds dialog functionalities to Bevy, akin to those of JRPG games!

# ✨ Features 
- Create beautiful visual novels and RPG titles using the Bevy game engine we know and love!
- `bevy_rpg` is made to be simple to the user (though complex inside) with not too many features.
- Write code without too many boilerplate! The following code is very pretty right? 
```rs
   Dialog::start(
        [
            d((&yuki, "Hiii haii haiii!")),
            d((
                ["I like this example", "Great enough", "Not so much"],
                Default::default()
            )),
        ]
        .into(),
        dialog_event,
    )
```
There are so much more you can do with the dialog syntax. Just see [basic.rs](./examples/basic.rs).

Oh, and. The dialog syntax is made by heavily abusing `From` and `Into` traits. Please do not go any far into the Rustdoc API documentation. 👀

## 📜 License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### 💁 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.