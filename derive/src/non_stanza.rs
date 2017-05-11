use syn::{self, Attribute, Ident, MetaItem, NestedMetaItem, Lit};
use quote::Tokens;


pub fn expand_derive_xmpp_event(input: &syn::DeriveInput) -> Result<Tokens, String> {
    let ident = &input.ident;
    let attrs = &input.attrs;
    let (event, typology) = extract_to_event(&attrs);
    let to_event = match syn::parse_expr(&event) {
        Ok(e) => e,
        Err(e) => panic!(e)
    };

    let (_, ty_generics, where_clause) = input.generics.split_for_impl();
    let dummy_const = Ident::new(format!("_IMPL_SERIALIZE_FOR_{}", ident).to_uppercase());

    let impl_id = match typology.as_ref() {
        "iq" => quote! {
            pub fn get_id(&self) -> &str {
                self.generic.get_id()
            }

            pub fn set_id<'a, T: ToString + ?Sized>(&'a mut self, id: &T) -> &'a mut Self {
                self.generic.set_id(id);
                self
            }
        },
        _ => quote!{}
    };

    let impl_type = match typology.as_ref() {
        "iq" => quote! {
            pub fn set_type<'a>(&'a mut self, iq_type: IqType) -> &'a mut Self {
                self.generic.set_type(iq_type);
                self
            }

            pub fn get_type(&self) -> IqType {
                self.generic.get_type()
            }
        },
        _ => quote!{}
    };

    let impl_to = match typology.as_ref() {
        "message" => quote! {
            pub fn set_to<'a>(&'a mut self, jid: Jid) -> &'a mut Self {
                self.generic.set_to(jid);
                self
            }

            pub fn get_to<'a >(&'a self) -> &'a Jid {
                self.generic.get_to()
            }
        },
        "iq" => quote! {
            pub fn set_to<'a>(&'a mut self, jid: Option<Jid>) -> &'a mut Self {
                self.generic.set_to(jid);
                self
            }

            pub fn get_to(&self) -> Option<&Jid> {
                self.generic.get_to()
            }
        },
        _ => quote!{}
    };

    let impl_from = match typology.as_ref() {
        "message"|
            "iq" => quote! {
                pub fn set_from<'a>(&'a mut self, jid: Option<Jid>) -> &'a mut Self {
                    self.generic.set_from(jid);
                    self
                }

                pub fn get_from(&self) -> Option<&Jid> {
                    self.generic.get_from()
                }
            },
        _ => quote!{}
    };

    let impl_to_generic = match typology.as_ref() {
        "presence" => quote! {
            pub fn to_presence(&self) -> Presence {
                Presence::from_element(self.to_element().unwrap()).unwrap()
            }
        },
        "message" => quote! {
            pub fn to_message(&self) -> GenericMessage {
                GenericMessage::from_element(self.to_element().unwrap()).unwrap()
            }
        },
        "iq" => quote! {
            pub fn to_generic(&self) -> GenericIq {
                GenericIq::from_element(self.to_element().unwrap()).unwrap()
            }
        },
        _ => quote!{}
    };
    let impl_block = quote! {
        impl #ident {
            #impl_to_generic
            #impl_id
            #impl_type
            #impl_to
            #impl_from
        }
    };
    let impl_block_event_trait = quote! {
        impl EventTrait for #ident #ty_generics #where_clause {
            fn to_event(&self) -> Event {
                #to_event
            }
        }
    };

    Ok(
        quote! {
            // #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
            const #dummy_const: () = {
                // extern crate xmpp_proto as _xmpp_proto;
                #impl_block_event_trait
                #impl_block
            };
        },
        )
}

fn extract_to_event(attrs: &Vec<Attribute>) -> (String, String) {
    for i in attrs {
        if i.name() == "non_stanza" {
            match i.value {
                MetaItem::List(_, ref meta_items) => {
                    let non_stanza_atts = parse_meta_items_non_stanza(&meta_items);
                    return non_stanza_atts.format()
                },
                _ => panic!("nooo")
            }
        } else if i.name() == "stanza" {
            match i.value {
                MetaItem::List(_, ref meta_items) => {
                    let stanza_atts = parse_meta_items_stanza(&meta_items);
                    return stanza_atts.format()
                },
                _ => panic!("nooo")
            }
        }
    }
    panic!("")
}
struct StanzaAttributes {
    event: String,
    transpile: bool,
    is: String
}

impl StanzaAttributes {
    fn new() -> StanzaAttributes {
        StanzaAttributes {
            event: String::from("_"),
            transpile: true,
            is: String::new()
        }
    }

    fn format(&self) -> (String, String) {
        match self.is.as_ref() {
            "iq" => {
                (format!("match self.generic.get_type() {{\
                    IqType::Result =>  return Event::Stanza(Box::new(StanzaEvent::IqResponseEvent(Box::new({event})))),
                    _ =>  return Event::Stanza(Box::new(StanzaEvent::IqRequestEvent(Box::new({event}))))
                }}", event=self.event), "iq".to_string())
            },
            "presence" => {
                (format!("return Event::Stanza(Box::new(StanzaEvent::PresenceEvent({event})))", event=self.event), "presence".to_string())
            },
            "message" => {
                (format!("return Event::Stanza(Box::new(StanzaEvent::MessageEvent(Box::new({event}))))", event=self.event), "message".to_string())
            },
            _ => (format!("return Event::Stanza(Box::new({}))", self.event), String::new())
        }
    }
}
struct NonStanzaAttributes {
    event: String,
    value: String
}

impl NonStanzaAttributes {
    fn new() -> NonStanzaAttributes {
        NonStanzaAttributes {
            event: String::from("self.clone()"),
            value: String::new(),
        }
    }

    fn format(&self) -> (String, String) {
        (format!("return Event::NonStanza(Box::new({}))", self.event), String::new())
    }
}

fn parse_meta_items_stanza(meta_items: &Vec<NestedMetaItem>) -> StanzaAttributes {
    let mut attr = StanzaAttributes::new();
    for i in meta_items {
        match *i {
            NestedMetaItem::MetaItem(MetaItem::NameValue(ref tag, Lit::Str(ref v,_))) => {
                if tag == "event" {
                    attr.event = v.to_string();
                } else if tag == "is" {
                    attr.is = v.to_lowercase();
                }
            },
            NestedMetaItem::MetaItem(MetaItem::Word(ref word)) => {
                if word == "no_transpile" {
                    attr.transpile = false;
                }
            },
            _ => {}
        }
    }

    attr.event = match attr.is.as_ref() {
        "message" if attr.transpile => attr.event.replace("_", "self.to_message()"),
        "iq" if attr.transpile => attr.event.replace("_", "self.to_generic()"),
        "presence" if attr.transpile => attr.event.replace("_", "self.to_presence()"),
        _ => attr.event.replace("_", "self.clone()")
    };

    attr
}
fn parse_meta_items_non_stanza(meta_items: &Vec<NestedMetaItem>) -> NonStanzaAttributes {
    let mut attr = NonStanzaAttributes::new();
    for i in meta_items {
        match *i {
            NestedMetaItem::MetaItem(MetaItem::NameValue(ref tag, Lit::Str(ref v,_))) => {
                if tag == "event" {
                    attr.event = v.to_string().replace("_", "Box::new(self.clone())");
                } else if tag == "value" {
                    attr.value = v.to_string();
                }
            },
            _ => {}
        }
    }
    attr
}
