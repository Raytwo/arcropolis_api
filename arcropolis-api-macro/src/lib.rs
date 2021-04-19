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
