use shopify_function::prelude::*;

pub mod cart_lines_discounts_generate_run;

#[typegen("schema.graphql")]
pub mod schema {
    
    #[query("src/cart_lines_discounts_generate_run.graphql")]
    pub mod cart_lines_discounts_generate_run {}
}

#[shopify_function]
fn run(
    input: schema::cart_lines_discounts_generate_run::Input,
) -> Result<schema::cart_lines_discounts_generate_run::Output> {
    cart_lines_discounts_generate_run::function(input)
}