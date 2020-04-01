//! Specifies various frameworks to link against. Note that this is something where you probably
//! only want to be compiling this project on macOS. ;P
//!
//! (it checks to see if it's macOS before emitting anything, but still)

fn main() {
    let target = std::env::var("TARGET").unwrap();

    println!("cargo:rustc-link-lib=framework=Foundation");
    
    if std::env::var("TARGET").unwrap().contains("-ios") {
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
}
