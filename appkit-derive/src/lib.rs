//! Macros used for `appkit-rs`. Mostly acting as `ShinkWrap`-esque forwarders.
//! Note that most of this is experimental!

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Derivces an `appkit::prelude::WinWrapper` block, which implements forwarding methods for things
/// like setting the window title, or showing and closing it. It currently expects that the wrapped
/// struct has `window` as the field holding the `Window` from `appkit-rs`.
///
/// Note that this expects that pointers to Window(s) should not move once created.
#[proc_macro_derive(WindowWrapper)]
pub fn impl_window_controller(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics appkit::prelude::WinWrapper for #name #ty_generics #where_clause {
            fn set_title(&self, title: &str) { self.window.set_title(title); }
            fn show(&self) { self.window.show(self); }
            fn close(&self) { self.window.close(); }
        }
    };

    TokenStream::from(expanded)
}
