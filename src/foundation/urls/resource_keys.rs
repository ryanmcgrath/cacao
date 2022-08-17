use crate::foundation::id;

/// Possible values for the `NSURLResourceKey::FileResourceType` key.
#[derive(Debug)]
pub enum NSURLFileResource {
    /// The resource is a named pipe.
    NamedPipe,

    /// The resource is a character special file.
    CharacterSpecial,

    /// The resource is a directory.
    Directory,

    /// The resource is a block special file.
    BlockSpecial,

    /// The resource is a regular file.
    Regular,

    /// The resource is a symbolic link.
    SymbolicLink,

    /// The resource is a socket.
    Socket,

    /// The resource’s type is unknown.
    Unknown,
}

/// Values that describe the iCloud storage state of a file.
#[derive(Debug)]
pub enum NSUbiquitousItemDownloadingStatus {
    /// A local copy of this item exists and is the most up-to-date version known to the device.
    Current,

    /// A local copy of this item exists, but it is stale. The most recent version will be downloaded as soon as possible.
    Downloaded,

    /// This item has not been downloaded yet. Initiate a download.
    NotDownloaded,
}

#[derive(Debug)]
pub enum NSURLResourceKey {
    IsApplication,
    IsScriptable,
    IsDirectory,
    ParentDirectoryURL,
    FileAllocatedSize,
    FileProtection,
    FileProtectionType,
    FileResourceIdentifier,
    FileResourceType(NSURLFileResource),
    FileSecurity,
    FileSize,
    IsAliasFile,
    IsPackage,
    IsRegularFile,
    PreferredIOBlockSize,
    TotalFileAllocatedSize,
    TotalFileSize,

    VolumeAvailableCapacity,
    VolumeAvailableCapacityForImportantUsage,
    VolumeAvailableCapacityForOpportunisticUsage,
    VolumeTotalCapacity,
    VolumeIsAutomounted,
    VolumeIsBrowsable,
    VolumeIsEjectable,
    VolumeIsEncrypted,
    VolumeIsInternal,
    VolumeIsJournaling,
    VolumeIsLocal,
    VolumeIsReadOnly,
    VolumeIsRemovable,
    VolumeIsRootFileSystem,

    IsMountTrigger,
    IsVolume,
    VolumeCreationDate,
    VolumeIdentifier,
    VolumeLocalizedFormatDescription,
    VolumeLocalizedName,
    VolumeMaximumFileSize,
    VolumeName,
    VolumeResourceCount,
    VolumeSupportsAccessPermissions,
    VolumeSupportsAdvisoryFileLocking,
    VolumeSupportsCasePreservedNames,
    VolumeSupportsCaseSensitiveNames,
    VolumeSupportsCompression,
    VolumeSupportsExclusiveRenaming,
    VolumeSupportsExtendedSecurity,
    VolumeSupportsFileCloning,
    VolumeSupportsHardLinks,
    VolumeSupportsImmutableFiles,
    VolumeSupportsJournaling,
    VolumeSupportsPersistentIDs,
    VolumeSupportsRenaming,
    VolumeSupportsRootDirectoryDates,
    VolumeSupportsSparseFiles,
    VolumeSupportsSwapRenaming,
    VolumeSupportsSymbolicLinks,
    VolumeSupportsVolumeSizes,
    VolumeSupportsZeroRuns,
    VolumeURLForRemounting,
    VolumeURL,
    VolumeUUIDString,

    IsUbiquitousItem,
    UbiquitousSharedItemMostRecentEditorNameComponents,
    UbiquitousItemDownloadRequested,
    UbiquitousItemIsDownloading,
    UbiquitousItemDownloadingError,
    UbiquitousItemDownloadingStatus(NSUbiquitousItemDownloadingStatus),
    UbiquitousItemIsUploaded,
    UbiquitousItemIsUploading,
    UbiquitousItemUploadingError,
    UbiquitousItemHasUnresolvedConflicts,
    UbiquitousItemContainerDisplayName,
    UbiquitousSharedItemOwnerNameComponents,
    UbiquitousSharedItemCurrentUserPermissions,
    UbiquitousSharedItemCurrentUserRole,
    UbiquitousItemIsShared,
    UbiquitousSharedItemRole,
    UbiquitousSharedItemPermissions,

    ThumbnailDictionaryItem,

    KeysOfUnsetValues,
    QuarantineProperties,
    AddedToDirectoryDate,
    AttributeModificationDate,
    ContentAccessDate,
    ContentModificationDate,
    CreationDate,
    CustomIcon,
    DocumentIdentifier,
    EffectiveIcon,
    GenerationIdentifier,
    HasHiddenExtension,
    IsExcludedFromBackup,
    IsExecutable,
    IsHidden,
    IsReadable,
    IsSymbolicLink,
    IsSystemImmutable,
    IsUserImmutable,
    IsWritable,
    LabelColor,
    LabelNumber,
    LinkCount,
    LocalizedLabel,
    LocalizedName,
    LocalizedTypeDescription,
    Name,
    Path,
    CanonicalPath,
    TagNames,
    ContentType,

    FileContentIdentifier,
    IsPurgeable,
    IsSparse,
    MayHaveExtendedAttributes,
    MayShareFileContent,
    UbiquitousItemIsExcludedFromSync,
    VolumeSupportsFileProtection,
}
