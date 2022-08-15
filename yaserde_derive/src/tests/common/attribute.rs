use std::collections::BTreeMap;
use syn::Attribute;
use crate::common::YaSerdeAttribute;

#[test]
fn parse_empty_attributes() {
  let attributes = vec![];
  let attrs = YaSerdeAttribute::parse(&attributes);

  assert_eq!(
    YaSerdeAttribute {
      attribute: false,
      default: None,
      default_namespace: None,
      flatten: false,
      namespaces: BTreeMap::new(),
      prefix: None,
      rename: None,
      skip_serializing: false,
      skip_serializing_if: None,
      text: false,
    },
    attrs
  );
}

#[test]
fn parse_attributes() {
  use proc_macro2::{Span, TokenStream};
  use std::str::FromStr;
  use syn::punctuated::Punctuated;
  use syn::token::Bracket;
  use syn::token::Pound;
  use syn::AttrStyle::Outer;
  use syn::{Ident, Path, PathArguments, PathSegment};

  let mut punctuated = Punctuated::new();
  punctuated.push(PathSegment {
    ident: Ident::new("yaserde", Span::call_site()),
    arguments: PathArguments::None,
  });

  let attributes = vec![Attribute {
    pound_token: Pound {
      spans: [Span::call_site()],
    },
    style: Outer,
    bracket_token: Bracket {
      span: Span::call_site(),
    },
    path: Path {
      leading_colon: None,
      segments: punctuated,
    },
    tokens: TokenStream::from_str("(attribute)").unwrap(),
  }];

  let attrs = YaSerdeAttribute::parse(&attributes);

  assert_eq!(
    YaSerdeAttribute {
      attribute: true,
      default: None,
      default_namespace: None,
      flatten: false,
      namespaces: BTreeMap::new(),
      prefix: None,
      rename: None,
      skip_serializing: false,
      skip_serializing_if: None,
      text: false,
    },
    attrs
  );
}

#[test]
fn only_parse_yaserde_attributes() {
  use proc_macro2::{Span, TokenStream};
  use std::str::FromStr;
  use syn::punctuated::Punctuated;
  use syn::token::Bracket;
  use syn::token::Pound;
  use syn::AttrStyle::Outer;
  use syn::{Ident, Path, PathArguments, PathSegment};

  let mut punctuated = Punctuated::new();
  punctuated.push(PathSegment {
    ident: Ident::new("serde", Span::call_site()),
    arguments: PathArguments::None,
  });

  let attributes = vec![Attribute {
    pound_token: Pound {
      spans: [Span::call_site()],
    },
    style: Outer,
    bracket_token: Bracket {
      span: Span::call_site(),
    },
    path: Path {
      leading_colon: None,
      segments: punctuated,
    },
    tokens: TokenStream::from_str("(flatten)").unwrap(),
  }];

  let attrs = YaSerdeAttribute::parse(&attributes);

  assert_eq!(
    YaSerdeAttribute {
      attribute: false,
      default: None,
      default_namespace: None,
      flatten: false,
      namespaces: BTreeMap::new(),
      prefix: None,
      rename: None,
      skip_serializing: false,
      skip_serializing_if: None,
      text: false,
    },
    attrs
  );
}

#[test]
fn parse_attributes_with_values() {
  use proc_macro2::{Span, TokenStream};
  use std::str::FromStr;
  use syn::punctuated::Punctuated;
  use syn::token::Bracket;
  use syn::token::Pound;
  use syn::AttrStyle::Outer;
  use syn::{Ident, Path, PathArguments, PathSegment};

  let mut punctuated = Punctuated::new();
  punctuated.push(PathSegment {
    ident: Ident::new("yaserde", Span::call_site()),
    arguments: PathArguments::None,
  });

  let attributes = vec![Attribute {
    pound_token: Pound {
      spans: [Span::call_site()],
    },
    style: Outer,
    bracket_token: Bracket {
      span: Span::call_site(),
    },
    path: Path {
      leading_colon: None,
      segments: punctuated,
    },
    tokens: TokenStream::from_str("(attribute, flatten, default_namespace=\"example\", namespace=\"example: http://example.org\")").unwrap(),
  }];

  let attrs = YaSerdeAttribute::parse(&attributes);

  let mut namespaces = BTreeMap::new();
  namespaces.insert(
    Some("example".to_string()),
    "http://example.org".to_string(),
  );

  assert_eq!(
    YaSerdeAttribute {
      attribute: true,
      default: None,
      default_namespace: Some("example".to_string()),
      flatten: true,
      namespaces,
      prefix: None,
      rename: None,
      skip_serializing: false,
      skip_serializing_if: None,
      text: false,
    },
    attrs
  );
}
