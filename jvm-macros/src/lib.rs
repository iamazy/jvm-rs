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
        impl<T: AsRef<[u8]>> InstructionReader<T> for #struct_ident {
            fn fetch_operands(&mut self, _reader: &mut Cursor<T>) {
                // do nothing
            }
        }
    };
    Ok(ret)
}

#[proc_macro_derive(Branch)]
pub fn branch_instruction_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match impl_brand(&input) {
        Ok(token_stream) => token_stream.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn impl_brand(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = &input.ident;
    let ret = quote! {
        impl<T: AsRef<[u8]>> InstructionReader<T> for #struct_ident {
            fn fetch_operands(&mut self, reader: &mut Cursor<T>) {
                self.offset = reader.get_i16() as i32;
            }
        }
    };
    Ok(ret)
}

#[proc_macro_derive(Index8)]
pub fn index8_instruction_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match impl_index8(&input) {
        Ok(token_stream) => token_stream.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn impl_index8(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = &input.ident;
    let ret = quote! {
        impl<T: AsRef<[u8]>> InstructionReader<T> for #struct_ident {
            fn fetch_operands(&mut self, reader: &mut Cursor<T>) {
                self.index = reader.get_u8() as usize;
            }
        }
    };
    Ok(ret)
}

#[proc_macro_derive(Index16)]
pub fn index16_instruction_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match impl_index16(&input) {
        Ok(token_stream) => token_stream.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn impl_index16(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = &input.ident;
    let ret = quote! {
        impl<T: AsRef<[u8]>> InstructionReader<T> for #struct_ident {
            fn fetch_operands(&mut self, reader: &mut Cursor<T>) {
                self.index = reader.get_u16() as u32;
            }
        }
    };
    Ok(ret)
}

#[proc_macro_derive(SymbolRef)]
pub fn symbolic_ref_instruction_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    match impl_symbol_ref(&input) {
        Ok(token_stream) => token_stream.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

fn impl_symbol_ref(input: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_ident = &input.ident;
    let ret = quote! {
        impl SymbolicRef for #struct_ident {
            fn resolved_class_ref(&mut self) -> anyhow::Result<()> {
                unsafe {
                    let class = self.constant_pool.as_mut().class.as_mut();
                    let class_loaded = class.loader.as_mut().load_class(&self.name.as_str())?;
                    if !class_loaded.is_accessible_to(class) {
                        return Err(anyhow!("java.lang.IllegalAccessError"));
                    }
                    self.class = Some(NonNull::from(class_loaded));
                    Ok(())
                }
            }
        }

        impl #struct_ident {
            pub fn resolved_class(&mut self) -> anyhow::Result<NonNull<Class>> {
                unsafe {
                    if self.class.is_none() {
                        self.resolved_class_ref()?;
                    }
                    Ok(self.class.unwrap().clone())
                }
            }
        }
    };
    Ok(ret)
}
