# Cacao iOS Support
This, unlike the macOS side of things, is much more alpha-quality. It does work, though - and this example will likely end up being a "kitchen sink" to figure things out with.

## Prerequisites

- Make sure that XCode is installed with iOS which you can get on [the app store](https://apps.apple.com/jp/app/xcode/id497799835)
- Make sure that the command line tools are installed and selected in on the Location tab of the XCode settings. Or run `sudo xcode-select --switch /Applications/Xcode.app`.
- Setup an [ios target](https://doc.rust-lang.org/rustc/platform-support/apple-ios.html) for rust, where you most likely should select `aarch64-apple-ios` in this day an age of Apple silicon.

## To run
Since this needs to run in an iOS simulator or on a device, you can't run it like a typical example. Follow the instructions below to give it a go:

- Start a simulator (Simulator.app).
- `cargo install cargo-bundle`
- `cargo bundle --example ios-beta --no-default-features --features uikit,autolayout --target aarch64-apple-ios`
- `xcrun simctl install booted target/aarch64-apple-ios/debug/examples/bundle/ios/ios-beta.app`
- `xcrun simctl launch --console booted com.cacao.ios-test`

## Current Support
Not much, but the basics of the scene delegate system work, along with view support, colors, and layout. Play around!

