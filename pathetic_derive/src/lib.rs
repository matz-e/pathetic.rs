extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(PartialOps)]
pub fn extend_struct(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let mut fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("expected a struct with fields"),
    }
    .iter()
    .map(|f| &f.ident)
    .flatten();

    let a = fields.next();
    let b = fields.next();
    let c = fields.next();
    let name = &input.ident;

    TokenStream::from(quote! {
        impl ops::Add<#name> for #name {
            type Output = #name;

            fn add(self, other: #name) -> #name {
                #name {
                    #a: self.#a + other.#a,
                    #b: self.#b + other.#b,
                    #c: self.#c + other.#c,
                }
            }
        }

        impl ops::AddAssign<#name> for #name {
            fn add_assign(&mut self, other: #name) {
                *self = #name {
                    #a: self.#a + other.#a,
                    #b: self.#b + other.#b,
                    #c: self.#c + other.#c,
                }
            }
        }

        impl ops::Div<f32> for #name {
            type Output = #name;

            fn div(self, num: f32) -> #name {
                #name {
                    #a: self.#a / num,
                    #b: self.#b / num,
                    #c: self.#c / num,
                }
            }
        }

        impl ops::Mul<#name> for f32 {
            type Output = #name;

            fn mul(self, other: #name) -> #name {
                #name {
                    #a: self * other.#a,
                    #b: self * other.#b,
                    #c: self * other.#c,
                }
            }
        }

        impl ops::Mul<f32> for #name {
            type Output = #name;

            fn mul(self, other: f32) -> #name {
                #name {
                    #a: other * self.#a,
                    #b: other * self.#b,
                    #c: other * self.#c,
                }
            }
        }

        impl ops::Neg for #name {
            type Output = #name;

            fn neg(self) -> #name {
                #name {
                    #a: -self.#a,
                    #b: -self.#b,
                    #c: -self.#c,
                }
            }
        }

        impl ops::Sub<#name> for #name {
            type Output = #name;

            fn sub(self, other: #name) -> #name {
                #name {
                    #a: self.#a - other.#a,
                    #b: self.#b - other.#b,
                    #c: self.#c - other.#c,
                }
            }
        }
    })
}
