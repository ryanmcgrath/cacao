//! Specifies various frameworks to link against. Note that this is something where you probably
//! only want to be compiling this project on macOS. ;P
//!
//! (it checks to see if it's macOS before emitting anything, but still)

fn main() {
    if std::env::var("TARGET").unwrap().contains("-apple") {
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");

        println!("cargo:rustc-link-lib=framework=Security");

        #[cfg(feature = "webview")]
        println!("cargo:rustc-link-lib=framework=WebKit");
        
        #[cfg(feature = "cloudkit")]
        println!("cargo:rustc-link-lib=framework=CloudKit");

        #[cfg(feature = "user-notifications")]
        println!("cargo:rustc-link-lib=framework=UserNotifications");
    }
}
