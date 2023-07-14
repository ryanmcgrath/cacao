# Contributing

Thanks for your interest in contributing to this project! Suggestions, bug reports, and pull requests and so on are cool, but keep in mind this is open source - there's currently no guarantee this project does much.

*Note:* Anyone who interacts with this project in any space, including but not
limited to this GitHub repository, must follow the [code of
conduct](https://github.com/ryanmcgrath/cacao/blob/trunk/code_of_conduct.md).


## Submitting bug reports

Have a look at the [issue tracker](https://github.com/ryanmcgrath/cacao/issues). If you can't find an issue (open or closed)
describing your problem (or a very similar one) there, please open a new issue with
the following details:

- Which versions of Rust and Cacao (and macOS/iOS build/device) are you using?
- Which feature flags are you using?
- What are you trying to accomplish?
- What is the full error you are seeing?
- How can this be reproduced?
  - Please quote as much of your code as needed to reproduce (best link to a
    public repository or [Gist])
  - Please post as much of your database schema as is relevant to your error

[issue tracker]: https://github.com/ryanmcgrath/cacao/issues
[Gist]: https://gist.github.com

Thank you!


## Submitting feature requests

If you can't find an issue (open or closed) describing your idea on the [issue
tracker], open an issue. Adding answers to the following
questions in your description is +1:

- What do you want to do, and how do you expect Cacao to support you with that?
- How might this be added to Cacao?
- What are possible alternatives?
- Are there any disadvantages?

Thank you!


## Contribute code to Cacao

### Setting up Cacao locally

1. Install Rust. Stable should be fine.
2. Clone this repository and open it in your favorite editor.
3. `cargo build`, or link it via your `Cargo.toml` to mess with it.

### Coding Style

Generally follow the [Rust Style Guide](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/guide/guide.md), enforced using [rustfmt](https://github.com/rust-lang-nursery/rustfmt).
In a few cases, though, it's fine to deviate - a good example is branching match trees.

To run rustfmt tests locally:

1. Use rustup to set Rust toolchain to the latest stable version of Rust.

2. Install the rustfmt and clippy by running
   ```
   rustup component add rustfmt-preview
   rustup component add clippy-preview
   ```

3. Run clippy nightly using cargo from the root of your Cacao repo.
   ```
   cargo +nightly clippy
   ```
   Each PR needs to compile without warning.

4. Run rustfmt nightly using cargo from the root of your Cacao repo.

   To see changes that need to be made, run

   ```
   cargo +nightly fmt --all -- --check
   ```

   If all code is properly formatted (e.g. if you have not made any changes),
   this should run without error or output.
   If your code needs to be reformatted,
   you will see a diff between your code and properly formatted code.
   If you see code here that you didn't make any changes to
   then you are probably running the wrong version of rustfmt.
   Once you are ready to apply the formatting changes, run

   ```
   cargo +nightly fmt --all
   ```

   You won't see any output, but all your files will be corrected.

You can also use rustfmt to make corrections or highlight issues in your editor.
Check out [their README](https://github.com/rust-lang/rustfmt) for details.


### Notes
This project prefers verbose naming, to a certain degree - UI code is read more often than written, so it's
worthwhile to ensure that it scans well. It also maps well to existing Cocoa/Cacao idioms and is generally preferred.
