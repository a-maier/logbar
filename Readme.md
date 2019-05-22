# Logbar

A log-friendly command-line progress bar.

Typical progress bar implementations update the progress report in
place, by going back and overwriting previous output. This allows for
beautiful progress displays, but becomes a problem when moving backwards
is not possible, for example when writing to a pipe. This crate's
progress bar never tries to modify what was printed before, so the
output can be piped directly to a log file.

# Usage

Add this to your Cargo.toml:

```toml
[dependencies]
logbar = "0.1"
```

# Examples


This example creates a default progress bar for a ten-step process:
```rust
let bar = logbar::ProgressBar::new(10);
// first step (10%) done
bar.inc(1);
// next three steps done
bar.inc(3);
// everything done
bar.finish();
```

We can also customise the style of the progress bar:
```rust
let style = Style::default()
    .width(80) // 80 characters wide
    .labels(false) // no XX% labels
    .tick('↓').bar('-') // rendered as ↓---↓---↓ etc.
    .indicator('█') // indicating the progress with '█' characters
        ;
let bar = logbar::ProgressBar::with_style(10, style);
bar.finish();
```
