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

    let campaigns = get_grouped_bundle_discounts();

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
            for campaign in &campaigns {
                println!("Checking campaign with Group A products:");
                let product_id = match line.merchandise() {
                    Merchandise::ProductVariant(variant) => variant.product().id(),
                    _ => continue,
                };
                if campaign.group_a_product_ids.iter().any(|&id| format!("gid://shopify/Product/{}", id) == *product_id) ||
                   campaign.group_b_product_ids.iter().any(|&id| format!("gid://shopify/Product/{}", id) == *product_id) {
                    return true;
                }
            }
            false
        });

    /*if !has_desired_product {
        return Ok(CartLinesDiscountsGenerateRunResult { operations: vec![] });
    }*/

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
    }else{
        operations.push(CartOperation::ProductDiscountsAdd( 
            ProductDiscountsAddOperation {
                selection_strategy: ProductDiscountSelectionStrategy::First,
                candidates: vec![ProductDiscountCandidate {
                    targets: vec![ProductDiscountCandidateTarget::CartLine(CartLineTarget {
                        id: max_cart_line.id().clone(),
                        quantity: None,
                    })],
                    message: Some(campaign.group_a_product_ids.iter().to_string()),
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

enum DiscountType {
    Percent,
    Dollar,
}

struct BundleDiscountCampaign {
    group_a_product_ids: Vec<i64>,
    quantity_needed_a: i32,
    group_b_product_ids: Vec<i64>,
    quantity_needed_b: i32,
    discount_type: DiscountType,
    discount_amount: f64,
    discount_message: String,
}

fn get_grouped_bundle_discounts() -> Vec<BundleDiscountCampaign> {
    vec![
        BundleDiscountCampaign {
            group_a_product_ids: vec![7328618152002, 7328089407554],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7328623067202, 7328621232194],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Hayden & Marley Bundle Save 15%!".to_string(),
        },
        BundleDiscountCampaign {
            group_a_product_ids: vec![7456189710402, 7456189481026, 7393003733058, 7351741218882, 7557712576578, 7557715591234, 7557719162946],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7455664144450, 7455662800962, 7455660146754, 7392838942786, 7362456485954, 7351415472194, 7559003603010, 7559005667394, 7559006978114, 7559016677442],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Mia & Ryann Bundle Save 15%!".to_string(),
        },
        BundleDiscountCampaign {
            group_a_product_ids: vec![7097113935938, 7519820218434, 7519813632066, 7378695585858, 7314269012034, 7519832440898, 7519826804802, 7378695979074, 7314269306946],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7324565831746, 7394177155138, 7394177744962],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Sophia & Emerson Bundle Save 15%!".to_string(),
        },
        BundleDiscountCampaign {
            group_a_product_ids: vec![7320236490818, 7320259985474, 7511312957506, 7511295524930, 7320254545986, 7511291199554, 7511314661442, 7511290282050, 7511313907778, 7511293558850],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7523781509186, 7524042014786, 7524043554882],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Hannah & Willa Bundle Save 15%!".to_string(),
        },
        BundleDiscountCampaign {
            group_a_product_ids: vec![7324430991426, 7511385374786, 7511379738690, 7511381966914, 7511384588354],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7351370645570, 7518924308546, 7518925291586, 7518926635074, 7518927028290],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Hadley & Luca Bundle Save 15%!".to_string(),
        },
        BundleDiscountCampaign {
            group_a_product_ids: vec![7523660496962, 7523663642690, 7523662561346, 7523663085634],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7320292819010, 7518190927938, 7518191157314, 7320294064194, 7518191353922, 7518183030850, 7518194237506, 7324589981762, 7518199808066, 7518200594498, 7518198857794, 7320297799746, 7518202626114, 7518203576386],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Sadie & Stella Bundle Save 15%!".to_string(),
        },
        BundleDiscountCampaign {
            group_a_product_ids: vec![7518957109314, 7518960910402],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7333725438018, 7519904268354, 7519904923714, 7519909412930],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Elsa & Frost Bundle Save 15%!".to_string(),
        },
        BundleDiscountCampaign {
            group_a_product_ids: vec![7521897381954, 7521906425922, 7521912193090, 7521912619074, 7521914421314, 7521916551234, 7521918058562],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7521940242498, 7521942241346, 7521942569026, 7521942962242, 7521943388226, 7521943584834, 7521943879746],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Mini Explorer Top & Bottom Bundle Save 15%!".to_string(),
        },
        BundleDiscountCampaign {
            group_a_product_ids: vec![7554343338050, 7349617885250, 7554326265922, 7554356871234],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7554407694402, 7555579150402, 7554395897922, 7555576561730],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Avery & Parker Bundle Save 15%!".to_string(),
        },
        BundleDiscountCampaign {
            group_a_product_ids: vec![7555594584130, 7555586523202, 7555601662018],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7555712286786, 7555684859970, 7555707338818, 7555713695810],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Bailey & Bree Bundle Save 15%!".to_string(),
        },
        BundleDiscountCampaign {
            group_a_product_ids: vec![7380366000194, 7511401365570, 7511403954242, 7511404642370, 7511405199426],
            quantity_needed_a: 1,
            group_b_product_ids: vec![7495154368578, 7495161020482],
            quantity_needed_b: 1,
            discount_type: DiscountType::Percent,
            discount_amount: 15.0,
            discount_message: "Evie & Ellis Bundle Save 15%!".to_string(),
        },
    ]
}