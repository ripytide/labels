/*
Copyright 2022 James Forster

This file is part of labels.

labels is free software: you can redistribute it and/or
modify it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

labels is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with labels. If not, see <https://www.gnu.org/licenses/>.
*/

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
