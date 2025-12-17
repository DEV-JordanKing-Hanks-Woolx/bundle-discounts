use shopify_function::prelude::*;
use shopify_function::Result;

mod cart_lines_discounts_generate_run;
use cart_lines_discounts_generate_run::run;

#[shopify_function_target(query_path = "src/cart_lines_discounts_generate_run.graphql", schema_path = "schema.graphql")]
fn run(input: input::ResponseData) -> Result<output::FunctionRunResult> {
    // Your discount logic here
    let mut operations = vec![];
    
    // Example: Add a product discount
    for line in input.cart.lines {
        operations.push(output::Operation {
            product_discounts_add: Some(output::ProductDiscountsAdd {
                candidates: vec![output::ProductDiscount {
                    message: Some("20% OFF PRODUCT".to_string()),
                    targets: vec![output::ProductDiscountTarget {
                        cart_line: Some(output::CartLineTarget {
                            id: line.id,
                            quantity: None,
                        }),
                    }],
                    value: output::ProductDiscountValue {
                        percentage: Some(output::Percentage {
                            value: 20.0,
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                }],
                selection_strategy: output::SelectionStrategy::First,
            }),
            ..Default::default()
        });
    }
    
    Ok(output::FunctionRunResult {
        operations,
        ..Default::default()
    })
}