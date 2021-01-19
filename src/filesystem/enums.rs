//! Certain enums that are useful (response types, etc).

use crate::foundation::{NSInteger, NSUInteger};

pub enum ModalResponse {
    Ok,
    Continue,
    Canceled,
    Stopped,
    Aborted,
    FirstButtonReturned,
    SecondButtonReturned,
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
            e => { panic!("Unknown NSModalResponse sent back! {}", e); }
        }
    }
}

pub enum SearchPathDomainMask {
    User,
    Local,
    Network,
    Domain,
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

pub enum SearchPathDirectory {
    Applications,
    DemoApplications,
    DeveloperApplications,
    AdminApplications,
    Library,
    Developer,
    User,
    Documentation,
    Documents,
    CoreServices,
    AutosavedInformation,
    Desktop,
    Caches,
    ApplicationSupport,
    Downloads,
    InputMethods,
    Movies,
    Music,
    Pictures,
    PrinterDescription,
    SharedPublic,
    PreferencePanes,
    ApplicationScripts,
    ItemReplacement,
    AllApplications,
    AllLibraries,
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
