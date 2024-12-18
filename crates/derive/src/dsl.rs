use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Field, GenericArgument, Ident, ItemStruct, PathSegment, Type};

pub fn derive_api(item: TokenStream) -> TokenStream {
    let ItemStruct {
        attrs: _,
        vis: _,
        struct_token: _,
        ident,
        generics,
        fields,
        semi_token: _,
    } = parse_macro_input!(item as ItemStruct);

    let mut apis = vec![];

    for field in fields {
        DeriveFiled::new(field).derive(&mut apis);
    }

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            #(#apis)*
        }
    }
    .into()
}

#[derive(PartialEq, Debug)]
enum DeriveType {
    Vec,
    Animatable,
    Option,
    Unknown(String),
}

struct DeriveFiled {
    ident: Ident,
    root_type: Type,
    type_stack: Vec<DeriveType>,
}

impl DeriveFiled {
    fn new(field: Field) -> Self {
        DeriveFiled {
            ident: field.ident.expect("Unsupport tuple structure."),
            root_type: field.ty,
            type_stack: Default::default(),
        }
    }

    fn parse_generic_type(seg: &PathSegment) -> &Type {
        match &seg.arguments {
            syn::PathArguments::AngleBracketed(args) => {
                match args.args.first().expect("DSL derive inner error.") {
                    GenericArgument::Type(t) => {
                        return t;
                    }
                    _ => {
                        panic!("DSL derive inner error.")
                    }
                }
            }
            _ => panic!("DSL derive inner error."),
        }
    }

    fn parse_field_type(&mut self) {
        let mut current_type = &self.root_type;

        loop {
            match current_type {
                Type::Path(path) => {
                    if path.path.segments.len() != 1 {
                        self.type_stack.push(DeriveType::Unknown(
                            current_type.to_token_stream().to_string(),
                        ));

                        break;
                    }

                    let seg = path.path.segments.first().unwrap();

                    match seg.ident.to_string().as_str() {
                        "Option" => {
                            // only parse top level `Option` type.
                            if self.type_stack.is_empty() {
                                self.type_stack.push(DeriveType::Option);

                                current_type = Self::parse_generic_type(seg);

                                continue;
                            } else {
                                self.type_stack.push(DeriveType::Unknown(
                                    current_type.to_token_stream().to_string(),
                                ));

                                break;
                            }
                        }
                        "Vec" => {
                            // only parse: Vec<T> or Option<Vec<T>>.
                            if self.type_stack.is_empty()
                                || (self.type_stack.len() == 1
                                    && *self.type_stack.first().unwrap() == DeriveType::Option)
                                || (self.type_stack.len() == 1
                                    && *self.type_stack.first().unwrap() == DeriveType::Animatable)
                            {
                                self.type_stack.push(DeriveType::Vec);

                                current_type = Self::parse_generic_type(seg);
                            } else {
                                self.type_stack.push(DeriveType::Unknown(
                                    current_type.to_token_stream().to_string(),
                                ));

                                break;
                            }

                            continue;
                        }
                        "Animatable" => {
                            // only parse Animatable<T>, Vec<Animatable<T>> or Option<Animatable<T>>,
                            if self.type_stack.is_empty()
                                || (self.type_stack.len() == 1
                                    && *self.type_stack.first().unwrap() == DeriveType::Option)
                            {
                                self.type_stack.push(DeriveType::Animatable);

                                current_type = Self::parse_generic_type(seg);

                                continue;
                            } else {
                                self.type_stack.push(DeriveType::Unknown(
                                    current_type.to_token_stream().to_string(),
                                ));

                                break;
                            }
                        }
                        _ => {
                            self.type_stack.push(DeriveType::Unknown(
                                current_type.to_token_stream().to_string(),
                            ));

                            break;
                        }
                    }
                }
                _ => {
                    self.type_stack.push(DeriveType::Unknown(
                        current_type.to_token_stream().to_string(),
                    ));

                    break;
                }
            }
        }
    }

    fn content_type(&self) -> proc_macro2::TokenStream {
        assert!(self.type_stack.len() > 0);
        assert!(self.type_stack.len() < 4);

        let content_type_index = match self.type_stack.len() {
            1 => 0,
            2 => 1,
            3 => 2,
            _ => panic!("DSL derive inner error."),
        };

        match &self.type_stack[content_type_index] {
            DeriveType::Unknown(token_stream) => {
                return token_stream.parse().unwrap();
            }
            _ => {
                panic!("DSL derive inner error.");
            }
        }
    }

    fn derive(&mut self, apis: &mut Vec<proc_macro2::TokenStream>) {
        self.parse_field_type();

        let fn_name = &self.ident;

        let fn_name_animated = format_ident!("{}_animated", fn_name);

        let content_type = self.content_type();

        match self.type_stack.first().unwrap() {
            DeriveType::Vec => {
                assert_eq!(self.type_stack.len(), 2);
                apis.push(quote! {
                    pub fn #fn_name<V>(mut self, v: V) -> Self
                    where
                        V: crate::MapCollect<#content_type>,
                    {
                        self.#fn_name = v.map_collect();
                        self
                    }
                });
            }
            DeriveType::Animatable => {
                if self.type_stack.len() == 3 {
                    assert_eq!(self.type_stack[1], DeriveType::Vec);

                    apis.push(quote! {
                        pub fn #fn_name<V>(mut self, v: V) -> Self
                        where
                            V: crate::MapCollect<#content_type>,
                        {
                            self.#fn_name = Animatable::Constant(v.map_collect());
                            self
                        }
                    });
                } else {
                    apis.push(quote! {
                        pub fn #fn_name<V>(mut self, v: V) -> Self
                        where
                            #content_type: From<V>,
                        {
                            self.#fn_name = Animatable::Constant(v.into());
                            self
                        }
                    });
                }

                apis.push(quote! {
                    pub fn #fn_name_animated<S>(mut self, v: S) -> Self
                    where
                        S: ToOwned<Owned = String>
                    {
                        self.#fn_name = Animatable::Animated(v.to_owned());
                        self
                    }
                });
            }
            DeriveType::Option => {
                if self.type_stack.len() == 3 {
                    match self.type_stack[1] {
                        DeriveType::Vec => {
                            apis.push(quote! {
                                pub fn #fn_name<V>(mut self, v: V) -> Self
                                where
                                    V: crate::MapCollect<#content_type>,
                                {
                                    self.#fn_name = Some(v.map_collect());
                                    self
                                }
                            });
                        }
                        DeriveType::Animatable => {
                            apis.push(quote! {
                                pub fn #fn_name<V>(mut self, v: V) -> Self
                                where
                                    #content_type: From<V>,
                                {
                                    self.#fn_name = Some(Animatable::Constant(v.into()));
                                    self
                                }

                                pub fn #fn_name_animated<S>(mut self, v: S) -> Self
                                where
                                    S: ToOwned<Owned = String>
                                {
                                    self.#fn_name = Some(Animatable::Animated(v.to_owned()));
                                    self
                                }
                            });
                        }
                        _ => {}
                    }
                } else {
                    apis.push(quote! {
                        pub fn #fn_name<V>(mut self, v: V) -> Self
                        where
                            #content_type: From<V>,
                        {
                            self.#fn_name = Some(v.into());
                            self
                        }
                    });
                }
            }
            DeriveType::Unknown(_) => {
                apis.push(quote! {
                    pub fn #fn_name<V>(mut self, v: V) -> Self
                    where
                        #content_type: From<V>,
                    {
                        self.#fn_name = v.into();
                        self
                    }
                });
            }
        }
    }
}
