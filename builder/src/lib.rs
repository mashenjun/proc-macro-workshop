#![allow(unused_variables)]
#![feature(proc_macro_quote)]
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::spanned::Spanned;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    // println!("input is {}", input);
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    let ret = do_expand(&st);
    match ret {
        Ok(tk_stream) => tk_stream.into(),
        Err(err) => {
            // println!("err is {}", err);
            return err.to_compile_error().into();
        }
    }
}

//     pub struct CommandBuilder {
//         executable: Option<String>,
//         args: Option<Vec<String>>,
//         env: Option<Vec<String>>,
//         current_dir: Option<String>,
//     }
fn do_expand(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    // eprintln!("{:#?}", st.data);
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = &syn::Ident::new(builder_name_literal.as_str(), st.span());
    let struct_ident = &st.ident;
    // returned result is already a TokenStream, call can embed it in `quote` direcrrly
    let builder_struct_field_def = generate_builder_struc_fields_def(st)?;
    // returned result is a Vec of TokenStream, caller should expand it in `quote`
    let builder_struc_factory_init_clauses = generate_builder_struc_factory_init_clauses(st)?;
    // we need to build a new strcut
    let ret = quote! {
        pub struct #builder_name_ident {
            #builder_struct_field_def
        }

        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident {
                    #(#builder_struc_factory_init_clauses),*
                }
            }
        }
    };
    return Ok(ret.into());
}

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;

fn get_fields_form_derive_input(st: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = st.data
    {
        return Ok(named);
    };
    Err(syn::Error::new_spanned(
        st,
        "Must define on struct, not on enum",
    ))
}
fn generate_builder_struc_fields_def(
    st: &syn::DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let fields = get_fields_form_derive_input(st)?;
    let idents: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    let ret = quote! {
        #(#idents: std::option::Option<#types>),*
    };
    return Ok(ret);
}

fn generate_builder_struc_factory_init_clauses(
    st: &syn::DeriveInput,
) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let fields = get_fields_form_derive_input(st)?;
    let init_cluase: Vec<_> = fields
        .iter()
        .map(|f| {
            let ident = &f.ident;
            quote! {
                     #ident: std::option::Option::None
            }
        })
        .collect();
    return Ok(init_cluase);
}
