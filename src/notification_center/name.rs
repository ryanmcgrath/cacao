#[allow(non_camel_case_types)]

use crate::foundation::NSString;

/// An enum that wraps NSNotificationName.
///
/// Since this framework utilizes Objective-C, these are ultimately backed by `NSString`... but we
/// want them to be a bit more type-friendly and autocomplete-able.
#[derive(Copy, Clone, Debug)]
pub enum NotificationName {
    /// Posted when the audio engine config changes.
    ///
    /// When using this, you must register from an `AudioEngine` instance.
    ///
    /// From Apple's documentation:
    ///
    /// > _When the audio engine’s I/O unit observes a change to the audio input or output
    /// > hardware’s channel count or sample rate, the audio engine stops, uninitializes itself,
    /// > and issues this notification. The nodes remain attached and connected with previously
    /// > set formats. The app must reestablish connections if the connection formats need to change.
    /// >
    /// > The engine must not be deallocated from within the client's notification handler. Callback
    /// > happens on an internal dispatch queue and can deadlock while trying to teardown the engine
    /// > synchronously._
    AudioEngineConfigurationChange,

    /// Posted when an audio interruption occurs.
    AudioSessionInterruption,

    /// Posted when the system terminates the media server.
    ///
    /// Most apps don't need to utilize this, and should opt to instead use
    /// `AudioSessionMediaServicesWereReset`. According to Apple, you can use this if you need to
    /// take action for requests that come in before the server restarts.
    AudioSessionMediaServicesWereLost,

    /// Posted when the media server restarts.
    ///
    /// In very rare cases, the system may terminate and restart its media services daemon. Respond to
    /// these events by reinitializing audio objects such as players, recorders, converters, and so on.
    /// Reset your audio session category, options, and mode configuration. Only restart playback, recording, or
    /// processing when initiated by the user.
    AudioSessionMediaServicesWereReset,

    /// Posted when the system audio route changes.
    ///
    /// The info provided by this notification contains the `AVAudioSessionRouteChange` keys, which
    /// provide more information.
    ///
    /// This notification is posted on a background thread.
    ///
    /// See [Apple's documentation on responding to Audio Session Route Changes](https://developer.apple.com/documentation/avfoundation/avaudiosession/responding_to_audio_session_route_changes?language=objc) for more information on using this notification.
    AudioSessionRouteChange,

    /// Posted when primary audio from other apps starts or stops.
    ///
    /// Subscribe to this notification to ensure that the system notifies your app when optional secondary
    /// audio muting should begin or end. The system sends this notification to registered listeners who
    /// are currently in the foreground with an active audio session.
    ///
    /// The info provided contains a `AVAudioSessionSilenceSecondaryAudioHintType` value for the
    /// `AVAudioSessionSilenceSecondaryAudioHintTypeKey`. Use the audio hint type to determine if your secondary
    /// audio muting should begin or end.
    ///
    /// This notification is posted on the main thread.
    AudioSessionSilenceSecondaryAudioHint,

    /// Posted when unit component tags change.
    ///
    /// The notification object is the AVAudioUnitComponent object which has changed tags.
    ///
    /// @TODO: This one may be tricky. Revisit when we do AVFoundation.
    AudioUnitComponentTagsDidChange,

    /// Posted when the signed-in iCloud account has changed.
    ///
    /// This notification is sent by instances of `CloudKitContainer`. If none exist, you won't get
    /// this notification. It can be sent on any thread, according to Apple.
    ///
    /// If you get this notification, you should query for the current account status.
    CloudKitAccountChanged,

    ///
    CLKComplicationServerActiveComplicationsDidChange,

    ///
    CNContactStoreDidChange,

    ///
    EKEventStoreChanged,

    ///
    HKUserPreferencesDidChange,

    ///
    HMCharacteristicPropertySupportsEvent,

    ///
    NSBundleResourceRequestLowDiskSpace,

    ///
    NSCalendarDayChanged,

    ///
    NSExtensionHostDidBecomeActive,

    ///
    NSExtensionHostDidEnterBackground,

    ///
    NSExtensionHostWillEnterForeground,

    ///
    NSExtensionHostWillResignActive,

    ///
    NSFileHandleConnectionAccepted,

    ///
    NSFileHandleDataAvailable,

    ///
    NSFileHandleReadToEndOfFileCompletion,

    ///
    NSHTTPCookieManagerAcceptPolicyChanged,

    ///
    NSHTTPCookieManagerCookiesChanged,

    ///
    NSManagedObjectContextDidSave,

    ///
    NSManagedObjectContextObjectsDidChange,

    ///
    NSManagedObjectContextWillSave,

    ///
    NSMetadataQueryDidFinishGathering,

    ///
    NSMetadataQueryDidStartGathering,

    ///
    NSMetadataQueryDidUpdate,

    ///
    NSMetadataQueryGatheringProgress,

    ///
    NSPersistentStoreCoordinatorStoresDidChange,

    ///
    NSPersistentStoreCoordinatorStoresWillChange,

    ///
    NSPersistentStoreCoordinatorWillRemoveStore,

    ///
    NSProcessInfoPowerStateDidChange,

    ///
    NSSystemClockDidChange,

    ///
    NSSystemTimeZoneDidChange,

    ///
    NSURLCredentialStorageChanged,

    ///
    NSUbiquityIdentityDidChange,

    ///
    NSUndoManagerCheckpoint,

    ///
    NSUndoManagerDidCloseUndoGroup,

    ///
    NSUndoManagerDidOpenUndoGroup,

    ///
    NSUndoManagerDidRedoChange,

    ///
    NSUndoManagerDidUndoChange,

    ///
    NSUndoManagerWillCloseUndoGroup,

    ///
    NSUndoManagerWillRedoChange,

    ///
    NSUndoManagerWillUndoChange,

    ///
    PKPassLibraryDidChange,

    ///
    PKPassLibraryRemotePaymentPassesDidChange,

    ///
    UIAccessibilityAnnouncementDidFinish,

    ///
    UIAccessibilityElementFocused,

    ///
    WKAudioFilePlayerItemDidPlayToEndTime,

    ///
    WKAudioFilePlayerItemFailedToPlayToEndTime,

    ///
    WKAudioFilePlayerItemTimeJumped,

    ///
    ABPeoplePickerDisplayedPropertyDidChange,

    ///
    ABPeoplePickerGroupSelectionDidChange,

    ///
    ABPeoplePickerNameSelectionDidChange,

    ///
    ABPeoplePickerValueSelectionDidChange,

    ///
    ACAccountStoreDidChange,

    ///
    AVAssetChapterMetadataGroupsDidChange,

    ///
    AVAssetContainsFragmentsDidChange,

    ///
    AVAssetDurationDidChange,

    ///
    AVAssetMediaSelectionGroupsDidChange,

    ///
    AVAssetTrackSegmentsDidChange,

    ///
    AVAssetTrackTimeRangeDidChange,

    ///
    AVAssetTrackTrackAssociationsDidChange,

    ///
    AVAssetWasDefragmented,

    ///
    AVCaptureDeviceWasConnected,

    ///
    AVCaptureDeviceWasDisconnected,

    ///
    AVCaptureInputPortFormatDescriptionDidChange,

    ///
    AVCaptureSessionDidStartRunning,

    ///
    AVCaptureSessionDidStopRunning,

    ///
    AVCaptureSessionRuntimeError,

    ///
    AVFragmentedMovieContainsMovieFragmentsDidChange,

    ///
    AVFragmentedMovieDurationDidChange,

    ///
    AVFragmentedMovieTrackSegmentsDidChange,

    ///
    AVFragmentedMovieTrackTimeRangeDidChange,

    ///
    AVFragmentedMovieTrackTotalSampleDataLengthDidChange,

    ///
    AVFragmentedMovieWasDefragmented,

    ///
    AVPlayerItemDidPlayToEndTime,

    ///
    AVPlayerItemFailedToPlayToEndTime,

    ///
    AVPlayerItemNewAccessLogEntry,

    ///
    AVPlayerItemNewErrorLogEntry,

    ///
    AVPlayerItemPlaybackStalled,

    ///
    AVPlayerItemTimeJumped,

    ///
    AVSampleBufferDisplayLayerFailedToDecode,

    ///
    CWBSSIDDidChange,

    ///
    CWCountryCodeDidChange,

    ///
    CWLinkDidChange,

    ///
    CWLinkQualityDidChange,

    ///
    CWModeDidChange,

    ///
    CWPowerDidChange,

    ///
    CWSSIDDidChange,

    ///
    CWScanCacheDidUpdate,

    ///
    GCControllerDidConnect,

    ///
    GCControllerDidDisconnect,

    ///
    IKFilterBrowserFilterDoubleClick,

    ///
    IKFilterBrowserFilterSelected,

    ///
    IKFilterBrowserWillPreviewFilter,

    ///
    IOBluetoothHostControllerPoweredOff,

    ///
    IOBluetoothHostControllerPoweredOn,

    ///
    IOBluetoothL2CAPChannelPublished,

    ///
    IOBluetoothL2CAPChannelTerminated,

    ///
    MKAnnotationCalloutInfoDidChange,

    ///
    NEFilterConfigurationDidChange,

    ///
    NEVPNConfigurationChange,

    ///
    NEVPNStatusDidChange,

    ///
    NSAccessibilityAnnouncementRequested,

    ///
    NSAccessibilityAnnouncementKey,

    ///
    NSAccessibilityPriorityKey,

    ///
    NSAccessibilityApplicationActivated,

    ///
    NSAccessibilityApplicationDeactivated,

    ///
    NSAccessibilityApplicationHidden,

    ///
    NSAccessibilityApplicationShown,

    ///
    NSAccessibilityCreated,

    ///
    NSAccessibilityDrawerCreated,

    ///
    NSAccessibilityFocusedUIElementChanged,

    ///
    NSAccessibilityFocusedWindowChanged,

    ///
    NSAccessibilityHelpTagCreated,

    ///
    NSAccessibilityLayoutChanged,

    ///
    NSAccessibilityUIElementsKey,

    ///
    NSAccessibilityMainWindowChanged,

    ///
    NSAccessibilityMoved,

    ///
    NSAccessibilityResized,

    ///
    NSAccessibilityRowCollapsed,

    ///
    NSAccessibilityRowCountChanged,

    ///
    NSAccessibilityRowExpanded,

    ///
    NSAccessibilitySelectedCellsChanged,

    ///
    NSAccessibilitySelectedChildrenChanged,

    ///
    NSAccessibilitySelectedChildrenMoved,

    ///
    NSAccessibilitySelectedColumnsChanged,

    ///
    NSAccessibilitySelectedRowsChanged,

    ///
    NSAccessibilitySelectedTextChanged,

    ///
    NSAccessibilitySheetCreated,

    ///
    NSAccessibilityTitleChanged,

    ///
    NSAccessibilityUIElementDestroyed,

    ///
    NSAccessibilityUnitsChanged,

    ///
    NSAccessibilityValueChanged,

    ///
    NSAccessibilityWindowCreated,

    ///
    NSAccessibilityWindowDeminiaturized,

    ///
    NSAccessibilityWindowMiniaturized,

    ///
    NSAccessibilityWindowMoved,

    ///
    NSAccessibilityWindowResized,

    ///
    NSAnimationProgressMark,

    ///
    NSAntialiasThresholdChanged,

    ///
    NSAppleEventManagerWillProcessFirstEvent,

    ///
    NSApplicationDidBecomeActive,

    ///
    NSApplicationDidChangeOcclusionState,

    ///
    NSApplicationDidChangeScreenParameters,

    ///
    NSApplicationDidFinishLaunching,

    ///
    NSApplicationDidFinishRestoringWindows,

    ///
    NSApplicationDidHide,

    ///
    NSApplicationDidResignActive,

    ///
    NSApplicationDidUnhide,

    ///
    NSApplicationDidUpdate,

    ///
    NSApplicationWillBecomeActive,

    ///
    NSApplicationWillFinishLaunching,

    ///
    NSApplicationWillHide,

    ///
    NSApplicationWillResignActive,

    ///
    NSApplicationWillTerminate,

    ///
    NSApplicationWillUnhide,

    ///
    NSApplicationWillUpdate,

    ///
    NSBrowserColumnConfigurationDidChange,

    ///
    NSClassDescriptionNeededForClass,

    ///
    NSColorListDidChange,

    ///
    NSColorPanelColorDidChange,

    ///
    NSColorPanel,

    ///
    NSComboBoxSelectionDidChange,

    ///
    NSComboBoxSelectionIsChanging,

    ///
    NSComboBoxWillDismiss,

    ///
    NSComboBoxWillPopUp,

    ///
    NSContextHelpModeDidActivate,

    ///
    NSContextHelpModeDidDeactivate,

    ///
    NSControlTextDidBeginEditing,

    ///
    NSControlTextDidChange,

    ///
    NSControlTextDidEndEditing,

    ///
    NSControlTintDidChange,

    ///
    NSDrawerDidClose,

    ///
    NSDrawerDidOpen,

    ///
    NSDrawerWillClose,

    ///
    NSDrawerWillOpen,

    ///
    NSFontCollectionDidChange,

    ///
    NSFontSetChanged,

    ///
    NSImageRepRegistryDidChange,

    ///
    NSMenuDidAddItem,

    ///
    NSMenuDidBeginTracking,

    ///
    NSMenuDidChangeItem,

    ///
    NSMenuDidEndTracking,

    ///
    NSMenuDidRemoveItem,

    ///
    NSMenuDidSendAction,

    ///
    NSMenuWillSendAction,

    ///
    NSOutlineViewColumnDidMove,

    ///
    NSOutlineViewColumnDidResize,

    ///
    NSOutlineViewItemDidCollapse,

    ///
    NSOutlineViewItemDidExpand,

    ///
    NSOutlineViewItemWillCollapse,

    ///
    NSOutlineViewItemWillExpand,

    ///
    NSOutlineViewSelectionDidChange,

    ///
    NSOutlineViewSelectionIsChanging,

    ///
    NSPersistentStoreDidImportUbiquitousContentChanges,

    ///
    NSPopUpButtonCellWillPopUp,

    ///
    NSPopUpButtonWillPopUp,

    ///
    NSPopoverDidClose,

    ///
    NSPopoverDidShow,

    ///
    NSPopoverWillClose,

    ///
    NSPopoverWillShow,

    ///
    NSPreferencePaneCancelUnselect,

    ///
    NSPreferencePaneDoUnselect,

    ///
    NSPreferencePaneSwitchToPane,

    ///
    NSPreferencePaneUpdateHelpMenu,

    ///
    NSPreferencePrefPaneIsAvailable,

    ///
    NSPreferredScrollerStyleDidChange,

    ///
    NSRuleEditorRowsDidChange,

    ///
    NSScreenColorSpaceDidChange,

    ///
    NSScrollViewDidEndLiveMagnify,

    ///
    NSScrollViewDidEndLiveScroll,

    ///
    NSScrollViewDidLiveScroll,

    ///
    NSScrollViewWillStartLiveMagnify,

    ///
    NSScrollViewWillStartLiveScroll,

    ///
    NSSpellCheckerDidChangeAutomaticCapitalization,

    ///
    NSSpellCheckerDidChangeAutomaticDashSubstitution,

    ///
    NSSpellCheckerDidChangeAutomaticPeriodSubstitution,

    ///
    NSSpellCheckerDidChangeAutomaticQuoteSubstitution,

    ///
    NSSpellCheckerDidChangeAutomaticSpellingCorrection,

    ///
    NSSpellCheckerDidChangeAutomaticTextReplacement,

    ///
    NSSplitViewDidResizeSubviews,

    ///
    NSSplitViewWillResizeSubviews,

    ///
    NSSystemColorsDidChange,

    ///
    NSTableViewColumnDidMove,

    ///
    NSTableViewColumnDidResize,

    ///
    NSTableViewSelectionDidChange,

    ///
    NSTableViewSelectionIsChanging,

    ///
    NSTextAlternativesSelectedAlternativeString,

    ///
    NSTextDidBeginEditing,

    ///
    NSTextDidChange,

    ///
    NSTextDidEndEditing,

    ///
    NSTextInputContextKeyboardSelectionDidChange,

    ///
    NSTextStorageDidProcessEditing,

    ///
    NSTextStorageWillProcessEditing,

    ///
    NSTextViewDidChangeSelection,

    ///
    NSTextViewDidChangeTypingAttributes,

    ///
    NSTextViewWillChangeNotifyingTextView,

    ///
    NSToolbarDidRemoveItem,

    ///
    NSToolbarWillAddItem,

    ///
    NSViewBoundsDidChange,

    ///
    NSViewDidUpdateTrackingAreas,

    ///
    NSViewFocusDidChange,

    ///
    NSViewFrameDidChange,

    ///
    NSViewGlobalFrameDidChange,

    ///
    NSWindowDidBecomeKey,

    ///
    NSWindowDidBecomeMain,

    ///
    NSWindowDidChangeBackingProperties,

    ///
    NSWindowDidChangeOcclusionState,

    ///
    NSWindowDidChangeScreen,

    ///
    NSWindowDidChangeScreenProfile,

    ///
    NSWindowDidDeminiaturize,

    ///
    NSWindowDidEndLiveResize,

    ///
    NSWindowDidEndSheet,

    ///
    NSWindowDidEnterFullScreen,

    ///
    NSWindowDidEnterVersionBrowser,

    ///
    NSWindowDidExitFullScreen,

    ///
    NSWindowDidExitVersionBrowser,

    ///
    NSWindowDidExpose,

    ///
    NSWindowDidMiniaturize,

    ///
    NSWindowDidMove,

    ///
    NSWindowDidResignKey,

    ///
    NSWindowDidResignMain,

    ///
    NSWindowDidResize,

    ///
    NSWindowDidUpdate,

    ///
    NSWindowWillBeginSheet,

    ///
    NSWindowWillClose,

    ///
    NSWindowWillEnterFullScreen,

    ///
    NSWindowWillEnterVersionBrowser,

    ///
    NSWindowWillExitFullScreen,

    ///
    NSWindowWillExitVersionBrowser,

    ///
    NSWindowWillMiniaturize,

    ///
    NSWindowWillMove,

    ///
    NSWindowWillStartLiveResize,

    ///
    NSWorkspaceAccessibilityDisplayOptionsDidChange,

    ///
    NSWorkspaceActiveSpaceDidChange,

    ///
    NSWorkspaceDidActivateApplication,

    ///
    NSWorkspaceDidChangeFileLabels,

    ///
    NSWorkspaceDidDeactivateApplication,

    ///
    NSWorkspaceDidHideApplication,

    ///
    NSWorkspaceDidLaunchApplication,

    ///
    NSWorkspaceDidMount,

    ///
    NSWorkspaceDidPerformFileOperation,

    ///
    NSWorkspaceDidRenameVolume,

    ///
    NSWorkspaceDidTerminateApplication,

    ///
    NSWorkspaceDidUnhideApplication,

    ///
    NSWorkspaceDidUnmount,

    ///
    NSWorkspaceDidWake,

    ///
    NSWorkspaceScreensDidSleep,

    ///
    NSWorkspaceScreensDidWake,

    ///
    NSWorkspaceSessionDidBecomeActive,

    ///
    NSWorkspaceSessionDidResignActive,

    ///
    NSWorkspaceWillLaunchApplication,

    ///
    NSWorkspaceWillPowerOff,

    ///
    NSWorkspaceWillSleep,

    ///
    NSWorkspaceWillUnmount,

    ///
    PDFDocumentDidBeginFind,

    ///
    PDFDocumentDidBeginPageFind,

    ///
    PDFDocumentDidBeginPageWrite,

    ///
    PDFDocumentDidBeginWrite,

    ///
    PDFDocumentDidEndFind,

    ///
    PDFDocumentDidEndPageFind,

    ///
    PDFDocumentDidEndPageWrite,

    ///
    PDFDocumentDidEndWrite,

    ///
    PDFDocumentDidFindMatch,

    ///
    PDFDocumentDidUnlock,

    ///
    PDFThumbnailViewDocumentEdited,

    ///
    PDFViewAnnotationHit,

    ///
    PDFViewAnnotationWillHit,

    ///
    PDFViewChangedHistory,

    ///
    PDFViewCopyPermission,

    ///
    PDFViewDisplayBoxChanged,

    ///
    PDFViewDisplayModeChanged,

    ///
    PDFViewDocumentChanged,

    ///
    PDFViewPageChanged,

    ///
    PDFViewPrintPermission,

    ///
    PDFViewScaleChanged,

    ///
    PDFViewSelectionChanged,

    ///
    PDFViewVisiblePagesChanged,

    ///
    KABDatabaseChanged,

    ///
    KABDatabaseChangedExternally,

    ///
    KQuartzFilterManagerDidAddFilter,

    ///
    KQuartzFilterManagerDidModifyFilter,

    ///
    KQuartzFilterManagerDidRemoveFilter,

    ///
    KQuartzFilterManagerDidSelectFilter,

    ///
    EAAccessoryDidConnect,

    ///
    EAAccessoryDidDisconnect,

    ///
    SKCloudServiceCapabilitiesDidChange,

    ///
    SKStorefrontIdentifierDidChange,

    ///
    UIAccessibilityAssistiveTouchStatusDidChange,

    ///
    UIAccessibilityBoldTextStatusDidChange,

    ///
    UIAccessibilityClosedCaptioningStatusDidChange,

    ///
    UIAccessibilityDarkerSystemColorsStatusDidChange,

    ///
    UIAccessibilityGrayscaleStatusDidChange,

    ///
    UIAccessibilityGuidedAccessStatusDidChange,

    ///
    UIAccessibilityHearingDevicePairedEarDidChange,

    ///
    UIAccessibilityInvertColorsStatusDidChange,

    ///
    UIAccessibilityMonoAudioStatusDidChange,

    ///
    UIAccessibilityReduceMotionStatusDidChange,

    ///
    UIAccessibilityReduceTransparencyStatusDidChange,

    ///
    UIAccessibilityShakeToUndoDidChange,

    ///
    UIAccessibilitySpeakScreenStatusDidChange,

    ///
    UIAccessibilitySpeakSelectionStatusDidChange,

    ///
    UIAccessibilitySwitchControlStatusDidChange,

    ///
    UIApplicationDidBecomeActive,

    ///
    UIApplicationDidEnterBackground,

    ///
    UIApplicationDidFinishLaunching,

    ///
    UIApplicationDidReceiveMemoryWarning,

    ///
    UIApplicationSignificantTimeChange,

    ///
    UIApplicationUserDidTakeScreenshot,

    ///
    UIApplicationWillEnterForeground,

    ///
    UIApplicationWillResignActive,

    ///
    UIApplicationWillTerminate,

    ///
    UIContentSizeCategoryDidChange,

    ///
    UIDeviceProximityStateDidChange,

    ///
    UIScreenBrightnessDidChange,

    ///
    UIScreenDidConnect,

    ///
    UIScreenDidDisconnect,

    ///
    UIScreenModeDidChange,

    ///
    UITableViewSelectionDidChange,

    ///
    UITextFieldTextDidBeginEditing,

    ///
    UITextFieldTextDidChange,

    ///
    UITextFieldTextDidEndEditing,

    ///
    UITextInputCurrentInputModeDidChange,

    ///
    UITextViewTextDidBeginEditing,

    ///
    UITextViewTextDidChange,

    ///
    UITextViewTextDidEndEditing,

    ///
    UIViewControllerShowDetailTargetDidChange,

    ///
    UIWindowDidBecomeHidden,

    ///
    UIWindowDidBecomeKey,

    ///
    UIWindowDidBecomeVisible,

    ///
    UIWindowDidResignKey,

    ///
    AVCaptureDeviceSubjectAreaDidChange,

    ///
    AVCaptureSessionInterruptionEnded,

    ///
    AVCaptureSessionWasInterrupted,

    ///
    MFMessageComposeViewControllerTextMessageAvailabilityDidChange,

    ///
    MPMediaLibraryDidChange,

    ///
    MPMusicPlayerControllerNowPlayingItemDidChange,

    ///
    MPMusicPlayerControllerPlaybackStateDidChange,

    ///
    MPMusicPlayerControllerVolumeDidChange,

    ///
    UIApplicationBackgroundRefreshStatusDidChange,

    ///
    UIDeviceBatteryLevelDidChange,

    ///
    UIDeviceBatteryStateDidChange,

    ///
    UIDeviceOrientationDidChange,

    ///
    UIDocumentStateChanged,

    ///
    UIKeyboardDidChangeFrame,

    ///
    UIKeyboardDidHide,

    ///
    UIKeyboardDidShow,

    ///
    UIKeyboardWillChangeFrame,

    ///
    UIKeyboardWillHide,

    ///
    UIKeyboardWillShow,

    ///
    UIMenuControllerDidHideMenu,

    ///
    UIMenuControllerDidShowMenu,

    ///
    UIMenuControllerMenuFrameDidChange,

    ///
    UIMenuControllerWillHideMenu,

    ///
    UIMenuControllerWillShowMenu,

    ///
    UIPasteboardChanged,

    ///
    UIPasteboardRemoved,

    ///
    UIApplicationProtectedDataDidBecomeAvailable,

    ///
    UIApplicationProtectedDataWillBecomeUnavailable,

    ///
    NSSpellCheckerDidChangeAutomaticTextCompletion,

    ///
    MPMusicPlayerControllerQueueDidChange,

    ///
    AVDisplayManagerModeSwitchEnd,

    ///
    AVDisplayManagerModeSwitchSettingsChanged,

    ///
    AVDisplayManagerModeSwitchStart,

    ///
    AVPlayerAvailableHDRModesDidChange,

    ///
    AVRouteDetectorMultipleRoutesDetectedDidChange,

    ///
    AVSampleBufferAudioRendererWasFlushedAutomatically,

    ///
    CTServiceRadioAccessTechnologyDidChange,

    ///
    GKPlayerAuthenticationDidChangeNotificationName,

    ///
    GKPlayerDidChangeNotificationName,

    ///
    NEDNSProxyConfigurationDidChange,

    ///
    NSPersistentStoreRemoteChange,

    ///
    SKStorefrontCountryCodeDidChange,

    ///
    WKAccessibilityReduceMotionStatusDidChange
}

impl From<NotificationName> for NSString<'_> {
    fn from(name: NotificationName) -> Self {
        match name {
            _ => NSString::no_copy("")
        }
    }
}
