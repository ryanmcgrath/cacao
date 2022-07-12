//! Emits linker flags depending on platforms and features.

fn main() {
    println!("cargo:rustc-link-lib=framework=Foundation");
   
    #[cfg(feature = "appkit")]
    println!("cargo:rustc-link-lib=framework=AppKit");
    
    #[cfg(feature = "uikit")]
    println!("cargo:rustc-link-lib=framework=UIKit");

    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=QuartzCore");
    println!("cargo:rustc-link-lib=framework=Security");

    #[cfg(feature = "webview")]
    println!("cargo:rustc-link-lib=framework=WebKit");
    
    #[cfg(feature = "cloudkit")]
    println!("cargo:rustc-link-lib=framework=CloudKit");

    #[cfg(feature = "user-notifications")]
    println!("cargo:rustc-link-lib=framework=UserNotifications");
    
    #[cfg(feature = "quicklook")]
    println!("cargo:rustc-link-lib=framework=QuickLook");
}
