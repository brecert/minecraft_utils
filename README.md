# minecraft_utils

Various utilities for interacting and working with minecraft.

Currently only includes a Mojang api interface.

## Examples

```rust
use minecraft_utils::mojang_api::{ Profile, get_username_uuid };

let uuid = get_username_uuid("brecert").unwrap();
let profile = Profile::fetch(&uuid).unwrap();

assert_eq!(
    profile.textures().skin.url,
    "http://textures.minecraft.net/texture/b8130282b80cc08872bfc858975350ab3f3fcd4b1d18717bfb5b7b838fce4eaa"
);
```

There's more examples in [examples](./examples).

> `cargo run --example get_info brecert`
