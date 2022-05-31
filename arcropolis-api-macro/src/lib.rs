use proc_macro::TokenStream;
use quote::quote;
use syn::parse::ParseStream;
use quote::ToTokens;

// Idea stolen from skyline-rs, fails to compile if i use the keyword macro directly
mod kw {
    syn::custom_keyword!(version);
}

struct APIVersionArgs {
    pub major: u32,
    pub minor: u32
}
#[proc_macro_attribute]
pub fn arc_callback(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);

    let ident = &func.sig.ident;
    let vis = &func.vis;

    quote!(
        #vis mod #ident {
            pub(crate) fn install<H: Into<::arcropolis_api::Hash40>>(hash: H, max_size: usize) {
                ::arcropolis_api::register_callback(hash.into(), max_size, callback)
            }

            extern "C" fn callback(
                hash: u64,
                data: *mut u8,
                size: usize,
                out_size: &mut usize
            ) -> bool {
                let data = unsafe { std::slice::from_raw_parts_mut(data, size) };

                match CB(hash, data) {
                    Some(size) => {
                        *out_size = size;
                        true
                    }
                    None => false
                }
            }

            const CB: fn(u64, &mut [u8]) -> Option<usize> = super::#ident;
        }

        #func
    ).into()
}

#[proc_macro_attribute]
pub fn stream_callback(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);

    let ident = &func.sig.ident;
    let vis = &func.vis;

    quote!(
        #vis mod #ident {
            pub(crate) fn install<H: Into<::arcropolis_api::Hash40>>(hash: H) {
                ::arcropolis_api::register_stream_callback(hash, callback)
            }

            extern "C" fn callback(
                hash: u64, out_path: *mut u8, out_size: &mut usize
            ) -> bool {
                fn inner<T: ::arcropolis_api::IntoStreamPath>(
                    callback: fn(u64) -> T,
                    hash: u64
                ) -> Option<(String, usize)> {
                    ::arcropolis_api::IntoStreamPath::into_stream_path(callback(hash))
                }

                match inner(super::#ident, hash) {
                    Some((path, size)) => {
                        let path = ::std::ffi::CString::new(path).unwrap();
                        let path_bytes = path.as_bytes_with_nul();

                        // definitely a buffer overflow here lol
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                path_bytes.as_ptr(),
                                out_path,
                                path_bytes.len()
                            )
                        };

                        *out_size = size;

                        true
                    }
                    None => false
                }
            }
        }

        #func
    ).into()
}

#[proc_macro_attribute]
pub fn ext_callback(_: TokenStream, item: TokenStream) -> TokenStream {
    let func = syn::parse_macro_input!(item as syn::ItemFn);

    let ident = &func.sig.ident;
    let vis = &func.vis;

    quote!(
        #vis mod #ident {
            pub(crate) fn install<H: Into<::arcropolis_api::Hash40>>(extension: H) {
                ::arcropolis_api::register_extension_callback(extension.into(), callback)
            }

            extern "C" fn callback(
                hash: u64,
                data: *mut u8,
                size: usize,
                out_size: &mut usize
            ) -> bool {
                let data = unsafe { std::slice::from_raw_parts_mut(data, size) };

                match CB(hash, data) {
                    Some(size) => {
                        *out_size = size;
                        true
                    }
                    None => false
                }
            }

            const CB: fn(u64, &mut [u8]) -> Option<usize> = super::#ident;
        }

        #func
    ).into()
}

impl syn::parse::Parse for APIVersionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(kw::version) {
            let meta: syn::MetaNameValue = input.parse()?;

            match meta.lit {
                syn::Lit::Str(string) => {
                    Ok(APIVersionArgs {
                        major: string.value().split(".").collect::<Vec<&str>>()[0].parse().unwrap(),
                        minor: string.value().split(".").collect::<Vec<&str>>()[1].parse().unwrap()
                    })
                }
                _ => panic!("Invalid literal, must be a string")
            }
        } else {
            panic!("Invalid argument!")
        }
    }
}

impl ToTokens for APIVersionArgs {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let major = self.major;
        let minor = self.minor;
        quote!(
            crate::require_api_version(#major, #minor)
        ).to_tokens(tokens);
    }
}

#[proc_macro_attribute]
pub fn arcrop_api(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr = syn::parse_macro_input!(attr as APIVersionArgs);
    let func = syn::parse_macro_input!(item as syn::ItemFn);
    let stmts = func.block.stmts.clone();
    let sig = &func.sig;
    let vis = &func.vis;

    quote!(
        #vis #sig {
            #attr;
            #(
                #stmts
            )*
        }

    ).into()
}