#![recursion_limit="128"]

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Uniform)]
pub fn uniform(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_uniform(&ast).into()
}

fn impl_uniform(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    let rname = syn::Ident::from(format!("Std140For{}", name));
    
    let fields = match &ast.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed {
                named,
                ..
            }),
            ..
        }) => named,
        _ => panic!(),
    };

    let aligned_fields = fields.iter().flat_map(|field| {
        let (a, f) = aligned_field(field);
        vec![a, f]
    });

    let field_names = fields.iter().map(|field| field.ident.unwrap());
    let field_names2 = fields.iter().map(|field| field.ident.unwrap());

    let dummy = syn::Ident::from(format!("_GLSL_LAYOUT_DUMMY_FOR_{}", name));

    quote! {
        #[allow(bad_style)]
        const #dummy: () = {
            extern crate glsl_layout as _glsl_layout;

            #[repr(C, align(16))]
            #[derive(Clone, Debug, Default)]
            struct #rname {#(
                #aligned_fields,
            )*}

            impl _glsl_layout::Uniform for #rname {
                type Align = _glsl_layout::align::Align16;
                type Std140 = #rname;

                fn align() -> _glsl_layout::align::Align16 { _glsl_layout::align::Align16 }
                fn std140(&self) -> #rname {
                    self.clone()
                }
            }

            impl _glsl_layout::Uniform for #name {
                type Align = _glsl_layout::align::Align16;
                type Std140 = #rname;

                fn align() -> _glsl_layout::align::Align16 { _glsl_layout::align::Align16 }
                fn std140(&self) -> #rname {
                    #rname {
                        #(#field_names: self.#field_names2.std140(),)*
                        ..Default::default()
                    }
                }
            }
        };
    }
}

fn aligned_field(field: &syn::Field) -> (syn::Field, syn::Field) {
    let name = field.ident.unwrap();
    let align = syn::Field {
        ty: syn::Type::Path(align_type_for(&field.ty)),
        ident: Some(format!("_align_{}", name).into()),
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        colon_token: Some(Default::default()),
    };

    let std140 = syn::Field {
        ty: syn::Type::Path(std140_type_for(&field.ty)),
        ..field.clone()
    };

    (align, std140)
}

fn align_type_for(aligned: &syn::Type) -> syn::TypePath {
    use std::iter::once;
    syn::TypePath {
        qself: Some(syn::QSelf {
            lt_token: Default::default(),
            ty: Box::new(aligned.clone()),
            position: 2,
            as_token: Some(Default::default()),
            gt_token: Default::default(),
        }),
        path: syn::Path {
            leading_colon: None,
            segments: once(syn::PathSegment::from("_glsl_layout"))
                .chain(once("Uniform".into()))
                .chain(once("Align".into()))
                .collect(),
        }
    }
}

fn std140_type_for(aligned: &syn::Type) -> syn::TypePath {
    use std::iter::once;
    syn::TypePath {
        qself: Some(syn::QSelf {
            lt_token: Default::default(),
            ty: Box::new(aligned.clone()),
            position: 2,
            as_token: Some(Default::default()),
            gt_token: Default::default(),
        }),
        path: syn::Path {
            leading_colon: None,
            segments: once(syn::PathSegment::from("_glsl_layout"))
                .chain(once("Uniform".into()))
                .chain(once("Std140".into()))
                .collect(),
        }
    }
}
