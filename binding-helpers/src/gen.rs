use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::io;
use std::path::Path;
use std::rc::Rc;

use bindgen::callbacks::{DeriveInfo, FieldInfo, ParseCallbacks};
use bindgen::FieldVisibilityKind;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use rust_format::{Formatter, RustFmt};

#[derive(Default, Debug)]
struct StructInfo {
    fields: HashMap<String, HashSet<String>>,
    names: HashSet<String>,
}

#[derive(Default, Debug)]
pub struct LayoutTests(Rc<RefCell<StructInfo>>);

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Format(rust_format::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Io(io) => write!(f, "IO error: {io}"),
            Error::Format(fmt) => write!(f, "Format error: {fmt}"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<rust_format::Error> for Error {
    fn from(e: rust_format::Error) -> Self {
        Self::Format(e)
    }
}

impl LayoutTests {
    pub fn new() -> (Self, Self) {
        let this = Self::default();

        (Self(Rc::clone(&this.0)), this)
    }

    pub fn generate_layout_tests(&self, output_path: impl AsRef<Path>) -> Result<(), Error> {
        let mut output = Vec::new();

        for (struct_name, fields) in &self.0.borrow().fields {
            if struct_name.contains("bindgen") {
                continue;
            }
            output.push(struct_layout_test(struct_name, fields));
        }

        let tokens = quote! { #(#output)* };

        // We can't use quote! here because it will reformat <> includes badly
        // and also doesn't seem to play nice with whatever cpp! expands to.
        let header = "
            use cpp::cpp;

            cpp! {{
                #include <stddef.h>
                #include <3ds.h>
            }}
        ";

        std::fs::write(&output_path, String::from(header) + &tokens.to_string())?;
        RustFmt::default().format_file(output_path)?;

        Ok(())
    }
}

fn struct_layout_test(
    struct_name: &str,
    field_names: &HashSet<String>,
) -> proc_macro2::TokenStream {
    let name = format_ident!("{struct_name}");

    let test_name = format_ident!("layout_test_{struct_name}");

    let mut field_tests = Vec::new();
    field_tests.push(assert_eq(quote!(size_of!(#name)), quote!(sizeof(#name))));
    field_tests.push(assert_eq(quote!(align_of!(#name)), quote!(alignof(#name))));

    for field in field_names {
        if field.contains("bindgen") {
            continue;
        }

        let field = if field == "type_" { "type" } else { field };

        let field = format_ident!("{field}");
        field_tests.push(assert_eq(
            quote!(size_of!(#name::#field)),
            quote!(sizeof(#name::#field)),
        ));
        field_tests.push(assert_eq(
            quote!(align_of!(#name::#field)),
            quote!(alignof(#name::#field)),
        ));
    }

    quote! {
        #[test]
        fn #test_name() {
            #(#field_tests);*
        }
    }
}

fn assert_eq(rust_lhs: TokenStream, cpp_rhs: TokenStream) -> TokenStream {
    quote! {
        assert_eq!(
            #rust_lhs,
            cpp!(unsafe [] -> usize as "size_t" { return #cpp_rhs; }),
            "{} != {}",
            stringify!(#rust_lhs),
            stringify!(#cpp_rhs),
        );
    }
}

impl ParseCallbacks for LayoutTests {
    fn add_derives(&self, info: &DeriveInfo<'_>) -> Vec<String> {
        self.0.borrow_mut().names.insert(info.name.to_string());
        Vec::new()
    }

    // We don't actually ever change visibility, but this allows us to keep track
    // of all the fields in the structs bindgen processes.
    fn field_visibility(&self, info: FieldInfo<'_>) -> Option<FieldVisibilityKind> {
        self.0
            .borrow_mut()
            .fields
            .entry(info.type_name.to_string())
            .or_default()
            .insert(info.field_name.to_string());

        None
    }
}
