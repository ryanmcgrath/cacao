# Rust Bindings for AppKit on macOS
This repository contains some _very_ exploratory, experimental bindings for `AppKit` on macOS. They aim to enable writing native macOS applications in pure Rust. There are currently no guarantees for anything, but you're welcome to clone and tinker.

- It relies on the Objective C runtime, so you should consider this a bridge and not "the way forward". With that said, something like this needs to exist to jumpstart things. Down the road I could totally see this being superseded. It could also be cool to see something like this used as a layer for rendering in other frameworks (e.g, [Bodil's vgtk](https://docs.rs/vgtk/0.2.1/vgtk/) or something).
- It attempts to mimic how you'd write things in native ObjC/Swift, by providing very clear hooks for lifecycle events.
- As it runs via the ObjC runtime, there are many `unsafe` blocks. You can question them, and feel free to suggest better ways to do it, but this library will never have no `unsafe` usage. Issues pertaining to total removal will be closed without question. If you want a Rust UI framework for the future, then follow what's happening over in Druid or something. If you'd like to do things now, natively, then feel free to consider tinkering with this.

I look at it like this: you realistically, for an app with a proper GUI, can't write 100% "safe" code today. You can get close, though - pick your poison.

## Can I use this now?
For now, you can clone this repository and link it into your `Cargo.toml` by path. I'm squatting the names on `crates.io`, as I (in time) will throw this up there, but only when it's at a point where there's reasonable expectation that things won't be changing around much.

If you're interested in seeing this in use in a shipping app, head on over to [subatomic](https://github.com/ryanmcgrath/subatomic/).

## Gotchas
Note that this framework expects that you're participating in code signing. Certain linked frameworks (`UserNotifications.framework`, etc) will not work if you're not.

## Etc
I assume I'll produce a better README at some point, but who knows. You can follow me over on [twitter](https://twitter.com/ryanmcgrath/) or [email me](mailto:ryan@rymc.io) with questions. Dual licensed MPL 2.0 and MIT.
