//! Specifies various frameworks to link against. Note that this is something where you probably
//! only want to be compiling this project on macOS. ;P
//!
//! (it checks to see if it's macOS before emitting anything, but still)

fn main() {
    if std::env::var("TARGET").unwrap().contains("-apple") {
        println!("cargo:rustc-link-lib=framework=Security");
        println!("cargo:rustc-link-lib=framework=WebKit");
        println!("cargo:rustc-link-lib=framework=UserNotifications");
    }
}
