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

`Notes:`  
You should edit `src/main.rs` to change the first line in `main`:  

```rust
// Change the default `true` to `false` to avoid regenerating a lot of txt files in lots of compile time
txt::generate_txt(true, 360, 280);
```

## Shortcuts

- `left` && `right` arrow key: backward && forward
- `q`  && `Ctrl-c`: quit
- `space` && `enter`: stop
- `r`: replay
