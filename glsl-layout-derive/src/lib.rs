#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro2::Span;

#[proc_macro_derive(Uniform)]
pub fn uniform(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = proc_macro2::TokenStream::from(input);

    let ast = syn::parse2(input).unwrap();

    proc_macro::TokenStream::from(impl_uniform(&ast))
}

fn impl_uniform(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    let rname = format_ident!("LayoutStd140{}", name);
    
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

    let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());
    let field_names2 = fields.iter().map(|field| field.ident.as_ref().unwrap());

    let dummy = format_ident!("_GLSL_LAYOUT_{}", name);

    quote! {
        #[allow(bad_style)]
        const #dummy: () = {
            extern crate glsl_layout as _glsl_layout;

            #[repr(C, align(16))]
            #[derive(Clone, Copy, Debug, Default)]
            pub struct #rname {#(
                #aligned_fields,
            )*}

            unsafe impl _glsl_layout::Std140 for #rname {}

            impl _glsl_layout::Uniform for #rname {
                type Align = _glsl_layout::align::Align16;
                type Std140 = #rname;

                fn std140(&self) -> #rname {
                    self.clone()
                }
            }

            impl _glsl_layout::Uniform for #name {
                type Align = _glsl_layout::align::Align16;
                type Std140 = #rname;

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
    let name = field.ident.as_ref().unwrap();
    let align = syn::Field {
        ty: syn::Type::Path(align_type_for(&field.ty)),
        ident: Some(format_ident!("_align_{}", name)),
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
            segments: once(syn::PathSegment::from(syn::Ident::new("_glsl_layout", Span::call_site())))
                .chain(once(syn::Ident::new("Uniform", Span::call_site()).into()))
                .chain(once(syn::Ident::new("Align", Span::call_site()).into()))
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
            segments: once(syn::PathSegment::from(syn::Ident::new("_glsl_layout", Span::call_site())))
                .chain(once(syn::Ident::new("Uniform".into(), Span::call_site()).into()))
                .chain(once(syn::Ident::new("Std140".into(), Span::call_site()).into()))
                .collect(),
        }
    }
}
