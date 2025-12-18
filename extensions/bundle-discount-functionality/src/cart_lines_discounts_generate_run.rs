use crate::schema::CartLineTarget;
use crate::schema::CartLinesDiscountsGenerateRunResult;
use crate::schema::CartOperation;
use crate::schema::Percentage;
use crate::schema::ProductDiscountCandidate;
use crate::schema::ProductDiscountCandidateTarget;
use crate::schema::ProductDiscountCandidateValue;
use crate::schema::ProductDiscountSelectionStrategy;
use crate::schema::ProductDiscountsAddOperation;
use crate::schema::cart_lines_discounts_generate_run::input::cart::lines::Merchandise;  
use super::schema;
use shopify_function::prelude::*;
use shopify_function::Result;

#[shopify_function]
fn cart_lines_discounts_generate_run(
    input: schema::cart_lines_discounts_generate_run::Input,
) -> Result<CartLinesDiscountsGenerateRunResult> {
    let max_cart_line = input
        .cart()
        .lines()
        .iter()
        .max_by(|a, b| {
            a.cost()
                .subtotal_amount()
                .amount()
                .partial_cmp(b.cost().subtotal_amount().amount())
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .ok_or("No cart lines found")?;
    
    let desired_product_id = "gid://shopify/Product/1"; // Replace with your target ID
    let has_desired_product = input
        .cart()
        .lines()
        .iter()
        .any(|line| {
            // Match on the Merchandise union
            match line.merchandise() {
                Merchandise::ProductVariant(variant) => variant.product().id() == desired_product_id,
                // Handle other variants if the union expands (e.g., custom products)
                _ => false,
            }
        });

    if !has_desired_product {
        return Ok(CartLinesDiscountsGenerateRunResult { operations: vec![] });
    }

    let mut operations = vec![];

    // Check if the discount has the PRODUCT class
    if has_desired_product {
        operations.push(CartOperation::ProductDiscountsAdd(
            ProductDiscountsAddOperation {
                selection_strategy: ProductDiscountSelectionStrategy::First,
                candidates: vec![ProductDiscountCandidate {
                    targets: vec![ProductDiscountCandidateTarget::CartLine(CartLineTarget {
                        id: max_cart_line.id().clone(),
                        quantity: None,
                    })],
                    message: Some("20% OFF PRODUCT".to_string()),
                    value: ProductDiscountCandidateValue::Percentage(Percentage {
                        value: Decimal(20.0),
                    }),
                    associated_discount_code: None,
                }],
            },
        ));
    }

    Ok(CartLinesDiscountsGenerateRunResult { operations })
}
