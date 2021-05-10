use proc_macro::TokenStream;
use quote::quote;

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
