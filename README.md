# bad-apple-rust
bad apple in terminal, using ratatui-rs && crossterm

## Example



## How to run it?

1. Clone the git repo

```bash
git clone https://github.com/jedsek/bad-apple-rust
cd bad-apple-rust
```

2. Use ffmpeg to generate every frames

```bash
ffmpeg -i video.mp4 images/image_%04d.jpg
```

3. run

```bash
cargo run --release
```

`Note_1:`  
You should edit `src/main.rs` to change the first line in `main`:  

```rust
// Change the default `true` to `false` to avoid regenerating a lot of txt files in lots of compile time
txt::generate_txt(true, 360, 280);
```

`Note_2:`  
Please note that if you want the best visual experience, you should set your terminal's `line-height` && `cell_width`:

`Wezterm (A configurable terminal)` Example:

```lua
-- ~/.config/wezterm/wezterm.lua
local config = {}

-- Make the cell looks more like a square rather than a rectangle
-- height:width = 1:2
config.line_height = 1;  
config.cell_width = 2;

return config
```

## Shortcuts

- `left` && `right` arrow key: backward && forward
- `q`  && `Ctrl-c`: quit
- `space` && `enter`: stop
- `r`: replay
