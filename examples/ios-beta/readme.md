# Cacao iOS Support
This, unlike the macOS side of things, is much more alpha-quality. It does work, though - and this example will likely end up being a "kitchen sink" to figure things out with.

## To run
Since this needs to run in an iOS simulator or on a device, you can't run it like a typical example. Follow the instructions below to give it a go:

- Start a simulator (Simulator.app).
- `cargo install cargo-bundle`
- `cargo bundle --example ios-beta --no-default-features --features uikit,autolayout --target x86_64-apple-ios`
- `xcrun simctl install booted target/x86_64-apple-ios/debug/examples/bundle/ios/cacao-ios-beta.app`
- `xcrun simctl launch --console booted com.cacao.ios-test`

## Current Support
Not much, but the basics of the scene delegate system work, along with view support, colors, and layout. Play around!
