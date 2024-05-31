Headless rendering based on https://github.com/bevyengine/bevy/blob/main/examples/app/headless_renderer.rs
Exporting engine internal state based on https://stackoverflow.com/a/75570011
  
Usage:
```
# Windowed render
cargo run -- '0,1>0,-1 -1,0>1,0'
# Headless render
cargo run -- '0,1>0,-1 -1,0>1,0' /tmp/test.png
```
  