use proc_macro::TokenStream;
use syn::spanned::Spanned;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = input;
    let st = syn::parse_macro_input!(input as syn::DeriveInput);
    match do_expand(&st) {
        Ok(token_stream)=>token_stream.into(),
        Err(e)=>e.to_compile_error().into(),
    }
}

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token![,]>;

fn get_fields_from_derive_input(st: &syn::DeriveInput)->syn::Result<&StructFields>{
    if let syn::Data::Struct(
        syn::DataStruct{
            fields: syn::Fields::Named(
                syn::FieldsNamed{
                    ref named,..
                }, ..
            ), 
            ..
        }
    ) = st.data{
        Ok(named)
    }else{
        Err(syn::Error::new_spanned(st, "Must Define On Struct, Not On Enum".to_string()))
    }
}


fn generate_builder_struct_fields_def(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let fileds = get_fields_from_derive_input(st)?;
    let idents: Vec<_> = fileds.iter().map(|f| &f.ident).collect();
    let types: Vec<_> = fileds.iter().map(|f| &f.ty).collect();

    let ret = quote::quote! {
        #(#idents: std::option::Option<#types>),*
    };
    Ok(ret)
}

fn generate_builder_struct_factory_init_clauses(st: &syn::DeriveInput) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let fileds = get_fields_from_derive_input(st)?;
    let init_clause: Vec<_> = fileds.iter().map(|f|{
        let ident = &f.ident;
        quote::quote! {
            #ident: std::option::Option::None
        }
    }).collect();
    Ok(init_clause)
    
}

fn do_expand(st: &syn::DeriveInput)->syn::Result<proc_macro2::TokenStream>{
    // eprintln!("{:#?}", st);
    let struct_name_literal = st.ident.to_string();
    let builder_name_literal = format!("{}Builder", struct_name_literal);
    let builder_name_ident = syn::Ident::new(builder_name_literal.as_str(), st.span());
    // 因为impl那里，#后只能识到st，不能识别到st.ident
    let struct_ident = st.ident.clone();

    let builder_struct_field_def = generate_builder_struct_fields_def(st)?;
    let builder_struct_factory_init_clauses = generate_builder_struct_factory_init_clauses(st)?;
    
    let ret = quote::quote! {
        pub struct #builder_name_ident {
            #builder_struct_field_def
        }

        impl #struct_ident {
            pub fn builder() -> #builder_name_ident {
                #builder_name_ident {
                    #(#builder_struct_factory_init_clauses),*
                }
            }
        }
    };
    return Ok(ret)
}