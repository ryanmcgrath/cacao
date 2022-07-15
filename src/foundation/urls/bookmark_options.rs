use crate::foundation::NSUInteger;

/// Options used when creating bookmark data.
#[derive(Copy, Clone, Debug)]
pub enum NSURLBookmarkCreationOption {
    /// Specifies that a bookmark created with this option should be created with minimal information.
    Minimal,

    /// Specifies that the bookmark data should include properties required to create Finder alias files.
    SuitableForBookmarkFile,

    /// Specifies that you want to create a security-scoped bookmark that, when resolved, provides a
    /// security-scoped URL allowing read/write access to a file-system resource.
    SecurityScoped,

    /// When combined with the NSURLBookmarkCreationOptions::SecurityScoped option, specifies that you
    /// want to create a security-scoped bookmark that, when resolved, provides a security-scoped URL allowing
    /// read-only access to a file-system resource.
    SecurityScopedReadOnly
}

impl From<NSURLBookmarkCreationOption> for NSUInteger {
    fn from(flag: NSURLBookmarkCreationOption) -> NSUInteger {
        match flag {
            NSURLBookmarkCreationOption::Minimal => 1u64 << 9,
            NSURLBookmarkCreationOption::SuitableForBookmarkFile => 1u64 << 10,
            NSURLBookmarkCreationOption::SecurityScoped => 1 << 11,
            NSURLBookmarkCreationOption::SecurityScopedReadOnly => 1 << 12
        }
    }
}

impl From<&NSURLBookmarkCreationOption> for NSUInteger {
    fn from(flag: &NSURLBookmarkCreationOption) -> NSUInteger {
        match flag {
            NSURLBookmarkCreationOption::Minimal => 1u64 << 9,
            NSURLBookmarkCreationOption::SuitableForBookmarkFile => 1u64 << 10,
            NSURLBookmarkCreationOption::SecurityScoped => 1 << 11,
            NSURLBookmarkCreationOption::SecurityScopedReadOnly => 1 << 12
        }
    }
}

/// Options used when resolving bookmark data.
#[derive(Debug)]
pub enum NSURLBookmarkResolutionOption {
    /// Specifies that no UI feedback should accompany resolution of the bookmark data.
    WithoutUI,

    /// Specifies that no volume should be mounted during resolution of the bookmark data.
    WithoutMounting,

    /// Specifies that the security scope, applied to the bookmark when it was created, should
    /// be used during resolution of the bookmark data.
    SecurityScoped
}

impl From<NSURLBookmarkResolutionOption> for NSUInteger {
    fn from(flag: NSURLBookmarkResolutionOption) -> NSUInteger {
        match flag {
            NSURLBookmarkResolutionOption::WithoutUI => 1u64 << 8,
            NSURLBookmarkResolutionOption::WithoutMounting => 1u64 << 9,
            NSURLBookmarkResolutionOption::SecurityScoped => 1 << 10
        }
    }
}
