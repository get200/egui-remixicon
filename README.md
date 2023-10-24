# egui_remixicon

Bundles [Remix Icon](https://remixicon.com/) with boilerplate to use in your egui app.

Got inspired by [egui_phosphor](https://github.com/amPerl/egui-phosphor), code uses the same structure.

## Installation

Add the crate as a dependency in Cargo.toml:

```toml
egui-remixicon = "0.1.0"
```

On startup, update the fonts in your egui context:

```rust
let mut fonts = egui::FontDefinitions::default();
egui_remixicon::add_to_fonts(&mut fonts);

cc.egui_ctx.set_fonts(fonts);
```

## Usage

Use the constants provided by the crate in your text:

```rust
ui.label(egui::RichText::new(format!("{} Heart", egui_remixicon::icons::HEARTS_FILL)).size(32.0));
```

## License

egui-remixicon is licensed under [MIT](LICENSE-MIT) OR [Apache-2.0](LICENSE-APACHE). Remix Icon are licensed under [Apache-2.0](https://github.com/Remix-Design/remixicon/blob/master/License).
