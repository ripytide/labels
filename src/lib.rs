#![crate_type = "proc-macro"]

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn tested(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn parent_tested(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn untested(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn trivial(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

//trivially made up of tested functions
#[proc_macro_attribute]
pub fn trivial_tested(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}

#[proc_macro_attribute]
pub fn not_a_fn(_attr: TokenStream, item: TokenStream) -> TokenStream {
	item
}
