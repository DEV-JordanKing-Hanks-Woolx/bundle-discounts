use crate::schema::CartLineTarget;
use crate::schema::CartLinesDiscountsGenerateRunResult;
use crate::schema::CartOperation;
use crate::schema::DiscountClass;
use crate::schema::OrderDiscountSelectionStrategy;
use crate::schema::OrderDiscountsAddOperation;
use crate::schema::OrderSubtotalTarget;
use crate::schema::Percentage;
use crate::schema::ProductDiscountCandidate;
use crate::schema::ProductDiscountCandidateTarget;
use crate::schema::ProductDiscountCandidateValue;
use crate::schema::ProductDiscountSelectionStrategy;
use crate::schema::ProductDiscountsAddOperation;

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

    let has_product_discount_class = input
        .discount()
        .discount_classes()
        .contains(&DiscountClass::Product);

    if  !has_product_discount_class {
        return Ok(CartLinesDiscountsGenerateRunResult { operations: vec![] });
    }

    let mut operations = vec![];
   
    // Check if the discount has the PRODUCT class
    if has_product_discount_class {
        println!("Ello!");
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
