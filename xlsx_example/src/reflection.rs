// extern crate proc_macro;

pub trait Reflection {
    fn field_names() -> Vec<String>;
}

// #[proc_macro_derive(Reflection)]
// pub fn reflection_derive(input: TokenStream) -> TokenStream {
//     unimplemented!()
// }
