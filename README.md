# 🦖 T-Rex

[![Crates.io](https://img.shields.io/crates/v/t-rex.svg)](https://crates.io/crates/t-rex)
[![Documentation](https://docs.rs/t-rex/badge.svg)](https://docs.rs/t-rex)
[![License](https://img.shields.io/crates/l/t-rex.svg)](https://github.com/yourusername/t-rex#license)

A powerful collection of EGUI widgets and components - as mighty as its namesake!

## Features

- 🎨 Custom styled widgets
- 🚀 High performance
- 📦 Easy to use
- 🦕 Prehistoric power, modern design

## Installation

Add this to your `Cargo.toml`:
```toml
[dependencies]
t-rex = "1.0.0"
eframe = "0.33.0"
egui = "0.33.0"
```

## Quick Start
```rust
use eframe::egui;
use t_rex::CustomButton;

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "T-Rex Demo",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("🦖 T-Rex Widgets");
            
            let button = CustomButton::new("Roar!");
            if button.ui(ui).clicked() {
                println!("T-Rex roared!");
            }
        });
    }
}
```

## Examples

Run the demo:
```bash
cargo run --example demo
```

## Widgets

- **CustomButton** - Enhanced button with custom styling
- **FancySlider** - Feature-rich slider component
- More coming soon!

## Contributing

Contributions are welcome! Feel free to open issues or submit PRs.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Why T-Rex?

Because like the mighty T-Rex, this library aims to be powerful, memorable, and make a big impact! 🦖