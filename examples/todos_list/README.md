# Tasks Example
This example implements a "full featured" macOS app, completely in Rust. Notably, it showcases the following:

- Working menu(s), with dispatchable actions.
- A cached, reusable `ListView`.
    - Cell configuration and styling.
    - Row actions: complete a task, mark a task as incomplete.
    - Basic animation support.
    - Self-sizing rows based on autolayout.
- Custom widget composition (see `preferences/toggle_option_view.rs`.
- Multiple windows.
- Toolbars per-window.
- Button and Toolbar Items, with actions.
- Running a window as a modal sheet.
- Autolayout to handle UI across the board.
- Message dispatch (a bit jank, but hey, that's fine for now).
- Standard "Preferences" screens
    - A general and advanced pane, with pane selection.
    - Looks correct on both Big Sur, as well as Catalina and earlier.

While the Cacao API is still subject to changes and revisions, this hopefully illustrates what's possible with everything as it currently exists, and provides an entry point for outside contributors to get their feet wet.
