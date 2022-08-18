//! Certain enums that are useful (response types, etc).

use crate::foundation::{NSInteger, NSUInteger};

/// Represents a modal response for macOS modal dialogs.
#[derive(Copy, Clone, Debug)]
pub enum ModalResponse {
    /// The user hit the "Ok" button.
    Ok,

    /// Continue.
    Continue,

    /// Canceled.
    Canceled,

    /// Stopped.
    Stopped,

    /// Aborted.
    Aborted,

    /// The first button in the dialog was clicked.
    FirstButtonReturned,

    /// The second button in the dialog was clicked.
    SecondButtonReturned,

    /// The third button in the dialog was clicked.
    ThirdButtonReturned
}

impl From<NSInteger> for ModalResponse {
    fn from(i: NSInteger) -> Self {
        match i {
            1 => ModalResponse::Ok,
            0 => ModalResponse::Canceled,
            1000 => ModalResponse::FirstButtonReturned,
            1001 => ModalResponse::SecondButtonReturned,
            1002 => ModalResponse::ThirdButtonReturned,
            -1000 => ModalResponse::Stopped,
            -1001 => ModalResponse::Aborted,
            -1002 => ModalResponse::Continue,

            // @TODO: Definitely don't panic here, wtf was I thinking?
            // Probably make this a ModalResponse::Unknown or something so a user can
            // gracefully handle.
            e => {
                panic!("Unknown NSModalResponse sent back! {}", e);
            }
        }
    }
}

/// Represents a type of search path used in file manager calls.
#[derive(Copy, Clone, Debug)]
pub enum SearchPathDomainMask {
    /// User files and folders.
    User,

    /// Local volume files and folders.
    Local,

    /// Netowrk files and folders.
    Network,

    /// Search all domains. Not typically used these days.
    Domain,

    /// Search all domains. Not typically used these days.
    AllDomains
}

impl From<SearchPathDomainMask> for NSUInteger {
    fn from(mask: SearchPathDomainMask) -> Self {
        match mask {
            SearchPathDomainMask::User => 1,
            SearchPathDomainMask::Local => 2,
            SearchPathDomainMask::Network => 4,
            SearchPathDomainMask::Domain => 8,
            SearchPathDomainMask::AllDomains => 0x0ffff
        }
    }
}

/// Represents a type of search path to use.
///
/// This enum is particularly useful for applications that need to exist both inside and outside of
/// the sandbox. For example: `SearchPathDirectory::Documents` will find the standard `Documents`
/// directory outside of the sandbox, but use the sandbox `Documents` directory in sandboxed
/// applications.
#[derive(Copy, Clone, Debug)]
pub enum SearchPathDirectory {
    /// The applications folder.
    Applications,

    /// Unsupported applications and demo versions. Not generally used these days.
    DemoApplications,

    /// Developer applications (_/Developer/Applications_). Not generally used these days.
    DeveloperApplications,

    /// System and network admin apps.
    AdminApplications,

    /// User-visible docs, support, and config files.
    Library,

    /// Dev resources. (_/Developer_)
    Developer,

    /// User home directories. (_/Users_)
    User,

    /// Documentation.
    Documentation,

    /// Documents directory.
    Documents,

    /// Core Services (_/System/Library/CoreServices_)
    CoreServices,

    /// User's autosaved documents (_/Library/Autosave Information_)
    AutosavedInformation,

    /// The current user's Desktop directory.
    Desktop,

    /// Discardable cache files. (_/Library/Caches_)
    Caches,

    /// App support files (_/Library/Application Support_)
    ApplicationSupport,

    /// The curent user's Downloads directory.
    Downloads,

    /// Input methods (_/Library/Input Methods_)
    InputMethods,

    /// The current user's Movies directory.
    Movies,

    /// The current user's Music directory.
    Music,

    /// The current user's pictures directory.
    Pictures,

    /// System PPD files (_/Library/Printers/PPDs_)
    PrinterDescription,

    /// The current user's public sharing directory.
    SharedPublic,

    /// The Preferences Pane directory, where system preferences files live.
    /// (_/Library/PreferencePanes_)
    PreferencePanes,

    /// The user scripts folder for the calling application
    /// (_~/Library/Application Scripts/<code-signing-id>_).
    ApplicationScripts,

    /// Constant used in creating a temp directory.
    ItemReplacement,

    /// All directories where apps can be stored.
    AllApplications,

    /// All directories where resources can be stored.
    AllLibraries,

    /// The Trash directory.
    Trash
}

impl From<SearchPathDirectory> for NSUInteger {
    fn from(directory: SearchPathDirectory) -> Self {
        match directory {
            SearchPathDirectory::Applications => 1,
            SearchPathDirectory::DemoApplications => 2,
            SearchPathDirectory::DeveloperApplications => 3,
            SearchPathDirectory::AdminApplications => 4,
            SearchPathDirectory::Library => 5,
            SearchPathDirectory::Developer => 6,
            SearchPathDirectory::User => 7,
            SearchPathDirectory::Documentation => 8,
            SearchPathDirectory::Documents => 9,
            SearchPathDirectory::CoreServices => 10,
            SearchPathDirectory::AutosavedInformation => 11,
            SearchPathDirectory::Desktop => 12,
            SearchPathDirectory::Caches => 13,
            SearchPathDirectory::ApplicationSupport => 14,
            SearchPathDirectory::Downloads => 15,
            SearchPathDirectory::InputMethods => 16,
            SearchPathDirectory::Movies => 17,
            SearchPathDirectory::Music => 18,
            SearchPathDirectory::Pictures => 19,
            SearchPathDirectory::PrinterDescription => 20,
            SearchPathDirectory::SharedPublic => 21,
            SearchPathDirectory::PreferencePanes => 22,
            SearchPathDirectory::ApplicationScripts => 23,
            SearchPathDirectory::ItemReplacement => 99,
            SearchPathDirectory::AllApplications => 100,
            SearchPathDirectory::AllLibraries => 101,
            SearchPathDirectory::Trash => 102
        }
    }
}
