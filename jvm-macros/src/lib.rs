use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(NoOperand)]
pub fn no_operand_instruction_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match impl_no_operand(&input) {
        Ok(token_stream) => token_stream.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn impl_no_operand(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = &input.ident;
    let ret = quote! {
        impl Instruction for #struct_ident {
            fn fetch_operands(&mut self, _reader: &mut BytesMut) {
                // do nothing
            }
        }
    };
    Ok(ret)
}
