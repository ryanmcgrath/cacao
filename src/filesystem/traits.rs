//! A trait that you can implement to handle open and save file dialogs. This more or less maps
//! over to `NSOpenPanel` and `NSSavePanel` handling.
pub trait OpenSaveController {
    /// Called when the user has entered a filename (typically, during saving). `confirmed`
    /// indicates whether or not they hit the save button.
    fn user_entered_filename(&self, _filename: &str, _confirmed: bool) {}

    /// Notifies you that the panel selection changed.
    fn panel_selection_did_change(&self) {}

    /// Notifies you that the user changed directories.
    fn did_change_to_directory(&self, _url: &str) {}

    /// Notifies you that the Save panel is about to expand or collapse because the user
    /// clicked the disclosure triangle that displays or hides the file browser.
    fn will_expand(&self, _expanding: bool) {}

    /// Determine whether the specified URL should be enabled in the Open panel.
    fn should_enable_url(&self, _url: &str) -> bool { true }
}

/// A trait you can implement for working with the underlying filesystem. This is important,
/// notably, because sandboxed applications have different working restrictions surrounding what
/// they can access.
pub trait FileManagerController {

}
