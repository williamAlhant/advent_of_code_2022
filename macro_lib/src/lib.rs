
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput
};

#[proc_macro_derive(Grid2D)]
pub fn derive_grid_2d(input: proc_macro::TokenStream) -> proc_macro::TokenStream {

    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl Grid2D for #name {
            fn get_node_from_id(&self, id: usize) -> Self::Node
            {
                Self::Node {
                    id: id,
                    data: self.data[id]
                }
            }

            fn get_node_left(&self, current: &Self::Node) -> Option<Self::Node>
            {
                let column = current.id % self.width;
                if column == 0 {
                    return None;
                }
                let dest_id = current.id - 1;
                Some(self.get_node_from_id(dest_id))
            }

            fn get_node_right(&self, current: &Self::Node) -> Option<Self::Node>
            {
                let column = current.id % self.width;
                if column == self.width - 1 {
                    return None;
                }
                let dest_id = current.id + 1;
                Some(self.get_node_from_id(dest_id))
            }

            fn get_node_up(&self, current: &Self::Node) -> Option<Self::Node>
            {
                let row = current.id / self.width;
                if row == 0 {
                    return None;
                }
                let dest_id = current.id - self.width;
                Some(self.get_node_from_id(dest_id))
            }

            fn get_node_down(&self, current: &Self::Node) -> Option<Self::Node>
            {
                let row = current.id / self.width;
                if row == self.height - 1 {
                    return None;
                }
                let dest_id = current.id + self.width;
                Some(self.get_node_from_id(dest_id))
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}
