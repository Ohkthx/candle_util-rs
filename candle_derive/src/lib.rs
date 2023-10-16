//! Derives for the `candle_util` crate.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Enables the `timestamp()` method. Returns the timestamp for the candle.
#[proc_macro_derive(Timestamp)]
pub fn timestamp_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Timestamp trait.
    TokenStream::from(quote! {
        impl Timestamp for #struct_name {
            fn timestamp(&self) -> u64 {
                self.start
            }
        }
    })
}

/// Enables the `start()` method. Returns the start timestamp for the candle.
#[proc_macro_derive(Start)]
pub fn start_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Start trait.
    TokenStream::from(quote! {
        impl Start for #struct_name {
            fn start(&self) -> u64 {
                self.start
            }
        }
    })
}

/// Enables the `open()` method. Returns the opening value for the candle.
#[proc_macro_derive(Open)]
pub fn open_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Open trait.
    TokenStream::from(quote! {
        impl Open for #struct_name {
            fn open(&self) -> f64 {
                self.open
            }
        }
    })
}

/// Enables the `close()` method. Returns the closing value for the candle.
#[proc_macro_derive(Close)]
pub fn close_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Close trait.
    TokenStream::from(quote! {
        impl Close for #struct_name {
            fn close(&self) -> f64 {
                self.close
            }
        }
    })
}

/// Enables the `low()` method. Returns the lowest value for the candle.
#[proc_macro_derive(Low)]
pub fn low_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Low trait.
    TokenStream::from(quote! {
        impl Low for #struct_name {
            fn low(&self) -> f64 {
                self.low
            }
        }
    })
}

/// Enables the `high()` method. Returns the highest value for the candle.
#[proc_macro_derive(High)]
pub fn high_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the High trait.
    TokenStream::from(quote! {
        impl High for #struct_name {
            fn high(&self) -> f64 {
                self.high
            }
        }
    })
}

/// Enables the `volume()` method. Returns the volume value for the candle.
#[proc_macro_derive(Volume)]
pub fn volume_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens and the name of the struct.
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    // Generate the implementation of the Volume trait.
    TokenStream::from(quote! {
        impl Volume for #struct_name {
            fn volume(&self) -> f64 {
                self.volume
            }
        }
    })
}
