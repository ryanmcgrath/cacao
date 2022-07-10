# Cacao Examples
This directory contains example code for apps written in cacao. To run an example, check out the list of commands below - some require certain features to be enabled.

## AutoLayout
An example that showcases layout out a view with AutoLayout. This requires the feature flag `autolayout` to be enabled, but it's defaulted for ease of use so doesn't need to be specified here. Platforms where AutoLayout is not supported will likely not work with this example.

`cargo run --example autolayout`

## Frame Layout
An example that showcases laying out with a more old school Frame-based approach. Platforms where AutoLayout are not supported will want to try this instead of the AutoLayout example.

**macOS:**
`cargo run --example frame_layout`

**Platforms lacking AutoLayout:**
`cargo run --example frame_layout --no-default-features --features appkit`

## Defaults
This example isn't GUI-specific, but showcases accessing `NSUserDefaults` from Rust for persisting basic data.

`cargo run --example defaults`

## Window
This example showcases creating a basic `Window`. This should run on all AppKit-supporting platforms.

`cargo run --example window`

## Window Controller
This example showcases creating a basic `WindowController`. This may run on all AppKit-supporting platforms.

`cargo run --example window_controller`

## Window Delegate
This example showcases creating a basic `WindowDelegate` to receive and handle events. This may run on all AppKit-supporting platforms.

`cargo run --example window_delegate`

## Text Input
This example showcases text input, and logs it to the underlying console. It's mostly a testbed to ensure that the backing widget for input behaves as expected.

`cargo run --example text_input`

## Calculator
A Rust-rendition of the macOS Calculator app.

`cargo run --example calculator`

## To-Do List
A "kitchen sink" example that showcases how to do more advanced things, such as cached reusable ListView components.

`cargo run --example todos_list`

## Browser
A _very_ basic web browser. Platforms that don't support WKWebView will likely not work with this example.

`cargo run --example browser --features webview`

## Webview Custom Protocol
This example showcases a custom protocol for the webview feature. Platforms that don't support WKWebView will likely not work with this example.

`cargo run --example webview_custom_protocol --features webview`

## iOS (Beta)
This example showcases how to build and run an iOS app in Rust. See the README in the `ios-beta` folder for instructions on how to run.
