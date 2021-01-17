//! Emits linker flags depending on platforms and features.
//!
//! (iOS/macOS only right now... maybe tvOS one day?)

fn main() {
    let target = std::env::var("TARGET").unwrap();

    println!("cargo:rustc-link-lib=framework=Foundation");
    
    if target.contains("-ios") {
        println!("cargo:rustc-link-lib=framework=UIKit");
    } else {
        println!("cargo:rustc-link-lib=framework=AppKit");
    }

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
