use std::collections::BTreeMap;

use proc_macro2::{Delimiter, Ident, token_stream::IntoIter, TokenStream, TokenTree};
use quote::quote;
use syn::__private::bool;
use syn::Attribute;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct YaSerdeAttribute {
  pub attribute: bool,
  pub default: Option<String>,
  pub default_namespace: Option<String>,
  pub flatten: bool,
  pub namespaces: BTreeMap<Option<String>, String>,
  pub prefix: Option<String>,
  pub rename: Option<String>,
  pub skip_serializing: bool,
  pub skip_serializing_if: Option<String>,
  pub text: bool,
}

fn get_value(iter: &mut IntoIter) -> Option<String> {
  match (iter.next(), iter.next()) {
    (Some(TokenTree::Punct(operator)),
      Some(TokenTree::Literal(value)))
    if operator.as_char() == '=' => {
      Some(value.to_string().replace('"', ""))
    }
    _ => None
  }
}

impl YaSerdeAttribute {
  pub fn parse(attrs: &[Attribute]) -> YaSerdeAttribute {
    let mut attribute = YaSerdeAttribute::default();

    for attr in attrs.iter().filter(|a| a.path.is_ident("yaserde")) {
      let mut attr_iter = attr.clone().tokens.into_iter();
      if let Some(TokenTree::Group(group)) = attr_iter.next() {
        if group.delimiter() == Delimiter::Parenthesis {
          let mut attr_iter = group.stream().into_iter();

          while let Some(item) = attr_iter.next() {
            attribute.set_value(item, &mut attr_iter)
          }
        }
      }
    }

    attribute
  }

  fn set_value(&mut self, item: TokenTree, attr_iter: &mut IntoIter) {
    if let TokenTree::Ident(ident) = item {
      match ident.to_string().as_str() {
        "attribute" => {
          self.attribute = true;
        }
        "default" => {
          self.default = get_value(attr_iter);
        }
        "default_namespace" => {
          self.default_namespace = get_value(attr_iter);
        }
        "flatten" => {
          self.flatten = true;
        }
        "namespace" => {
          if let Some(namespace) = get_value(attr_iter) {
            let splitted: Vec<&str> = namespace.split(": ").collect();
            if splitted.len() == 2 {
              self.namespaces.insert(Some(splitted[0].to_owned()), splitted[1].to_owned());
            }
            if splitted.len() == 1 {
              self.namespaces.insert(None, splitted[0].to_owned());
            }
          }
        }
        "prefix" => {
          self.prefix = get_value(attr_iter);
        }
        "rename" => {
          self.rename = get_value(attr_iter);
        }
        "skip_serializing" => {
          self.skip_serializing = true;
        }
        "skip_serializing_if" => {
          self.skip_serializing_if = get_value(attr_iter);
        }
        "text" => {
          self.text = true;
        }
        _ => {}
      }
    }
  }

  pub fn xml_element_name(&self, ident: &Ident) -> String {
    self.rename.clone().unwrap_or_else(|| ident.to_string())
  }

  pub fn prefix_namespace(&self) -> String {
    if self.default_namespace == self.prefix {
      "".to_string()
    } else {
      self
        .clone()
        .prefix
        .map_or("".to_string(), |prefix| prefix + ":")
    }
  }

  pub fn get_namespace_matching(
    &self,
    prefix: &Option<String>,
    element_namespace: TokenStream,
    element_name: TokenStream,
    take_root_prefix: bool,
  ) -> TokenStream {
    let configured_prefix = if take_root_prefix {
      self.prefix.clone()
    } else {
      prefix.clone()
    };

    let namespaces_matches: TokenStream = self
      .namespaces
      .iter()
      .filter_map(|(prefix, namespace)| {
        if configured_prefix.eq(prefix) {
          Some(quote!(#namespace => {}))
        } else {
          None
        }
      })
      .collect();

    quote!(
      if let Some(namespace) = #element_namespace {
        match namespace.as_str() {
          #namespaces_matches
          bad_namespace => {
            let msg =
              ::std::format!("bad namespace for {}, found {}", #element_name, bad_namespace);
            return Err(msg);
          }
        }
      }
    )
  }
}