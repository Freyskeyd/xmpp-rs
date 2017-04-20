#![recursion_limit="128"]
extern crate proc_macro;
extern crate serde_codegen_internals as internals;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(Iq)]
pub fn iq(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    
    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = impl_iq(&ast);
    
    // Return the generated impl
    gen.parse().unwrap()
}

mod non_stanza;

#[proc_macro_derive(XmppEvent, attributes(non_stanza, stanza))]
pub fn derive_xmpp_event(input: TokenStream) -> TokenStream {
    let input = syn::parse_derive_input(&input.to_string()).unwrap();
    match non_stanza::expand_derive_xmpp_event(&input) {
        Ok(expanded) => expanded.parse().unwrap(),
        Err(msg) => panic!(msg),
    }
}

fn impl_iq(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl #name {
            pub fn get_id(&self) -> String {
                self.generic.get_id()
            }

            pub fn set_id(self, id: &str) -> Self {
                self.generic.set_id(id);

                self
            }

            pub fn get_to(&self) -> Option<String> {
                self.generic.get_to()
            }

            pub fn get_from(&self) -> Option<Jid> {
                self.generic.get_from()
            }

            pub fn get_type(&self) -> IqType {
                self.generic.get_type()
            }

            pub fn set_type(self, t: IqType) -> Self {
                self.generic.set_type(t);

                self
            }
        }
    }
}
