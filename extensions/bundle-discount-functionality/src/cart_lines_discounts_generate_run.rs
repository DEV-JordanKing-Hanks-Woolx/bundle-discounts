use std::collections::HashMap;

// ================================ Customizable Settings ================================
const ENABLE_REBUY_FREE_GIFT: bool = true;

#[derive(Clone)]
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

struct RebuyFreeGiftSettings {
    threshold_subtotal: i64,
    gift_variant_ids: Vec<i64>,
    rebuy_attribution_key: String,
    rebuy_attribution_value: String,
    free_gift_message: String,
}

fn get_rebuy_settings() -> RebuyFreeGiftSettings {
    RebuyFreeGiftSettings {
        threshold_subtotal: 200,
        gift_variant_ids: vec![42262348300354, 42262345384002],
        rebuy_attribution_key: "_attribution".to_string(),
        rebuy_attribution_value: "Rebuy Tiered Progress Bar".to_string(),
        free_gift_message: "Free Gift for spending 200 {{currency}}".to_string(),
    }
}

// ================================ Data Structures ================================

#[derive(Clone)]
struct LineItem {
    id: usize,
    variant_id: i64,
    product_id: i64,
    quantity: i32,
    line_price: i64, // Price in cents
    line_price_changed: bool,
    properties: HashMap<String, String>,
    discount_message: Option<String>,
}

struct Cart {
    line_items: Vec<LineItem>,
    subtotal_price: i64, // In cents
    presentment_currency: String,
}

// ================================ Bundle Logic ================================

struct GroupData {
    ids: Vec<i64>,
    qty_needed: i32,
    items: Vec<usize>, // indices into cart.line_items
    total: i32,
}

struct GroupBundleSelector {
    group_a: GroupData,
    group_b: GroupData,
}

impl GroupBundleSelector {
    fn new(
        group_a_ids: Vec<i64>,
        qty_a_needed: i32,
        group_b_ids: Vec<i64>,
        qty_b_needed: i32,
    ) -> Self {
        Self {
            group_a: GroupData {
                ids: group_a_ids,
                qty_needed: qty_a_needed,
                items: Vec::new(),
                total: 0,
            },
            group_b: GroupData {
                ids: group_b_ids,
                qty_needed: qty_b_needed,
                items: Vec::new(),
                total: 0,
            },
        }
    }

    fn build(&mut self, cart: &Cart) {
        for (idx, li) in cart.line_items.iter().enumerate() {
            if li.line_price_changed {
                continue;
            }

            if self.group_a.ids.contains(&li.product_id) {
                self.group_a.items.push(idx);
                self.group_a.total += li.quantity;
            }
            if self.group_b.ids.contains(&li.product_id) {
                self.group_b.items.push(idx);
                self.group_b.total += li.quantity;
            }
        }
    }
}

struct DiscountApplicator {
    discount_type: DiscountType,
    discount_amount: f64,
    discount_message: String,
}

impl DiscountApplicator {
    fn new(discount_type: DiscountType, discount_amount: f64, discount_message: String) -> Self {
        Self {
            discount_type,
            discount_amount,
            discount_message,
        }
    }

    fn apply(&self, line_item: &mut LineItem) {
        let new_line_price = match self.discount_type {
            DiscountType::Percent => {
                (line_item.line_price as f64 * (1.0 - self.discount_amount / 100.0)) as i64
            }
            DiscountType::Dollar => {
                let discount_cents = (self.discount_amount * 100.0) as i64;
                (line_item.line_price - discount_cents * line_item.quantity as i64).max(0)
            }
        };

        line_item.line_price = new_line_price;
        line_item.line_price_changed = true;
        line_item.discount_message = Some(self.discount_message.clone());
    }
}

struct DiscountLoop {
    applicator: DiscountApplicator,
}

impl DiscountLoop {
    fn new(applicator: DiscountApplicator) -> Self {
        Self { applicator }
    }

    fn loop_items(&self, cart: &mut Cart, item_indices: &[usize], num_to_discount: i32) {
        let mut remaining = num_to_discount;

        for &idx in item_indices {
            if remaining <= 0 {
                break;
            }

            let item = &mut cart.line_items[idx];

            if item.quantity > remaining {
                // Split the line item
                let mut split_item = item.clone();
                split_item.quantity = remaining;
                split_item.line_price = (item.line_price / item.quantity as i64) * remaining as i64;
                
                item.quantity -= remaining;
                item.line_price = (item.line_price / (item.quantity + remaining) as i64) * item.quantity as i64;

                self.applicator.apply(&mut split_item);
                cart.line_items.insert(idx + 1, split_item);
                break;
            } else {
                self.applicator.apply(item);
                remaining -= item.quantity;
            }
        }
    }
}

fn run_grouped_bundle_discounts(cart: &mut Cart, campaigns: &[BundleDiscountCampaign]) {
    for campaign in campaigns {
        let mut selector = GroupBundleSelector::new(
            campaign.group_a_product_ids.clone(),
            campaign.quantity_needed_a,
            campaign.group_b_product_ids.clone(),
            campaign.quantity_needed_b,
        );
        selector.build(cart);

        if selector.group_a.total < selector.group_a.qty_needed
            || selector.group_b.total < selector.group_b.qty_needed
        {
            continue;
        }

        let num_pairs = ((selector.group_a.total / selector.group_a.qty_needed)
            .min(selector.group_b.total / selector.group_b.qty_needed)) as i32;

        if num_pairs <= 0 {
            continue;
        }

        let applicator = DiscountApplicator::new(
            campaign.discount_type.clone(),
            campaign.discount_amount,
            campaign.discount_message.clone(),
        );
        let looper = DiscountLoop::new(applicator);

        looper.loop_items(
            cart,
            &selector.group_a.items,
            selector.group_a.qty_needed * num_pairs,
        );
        
        let applicator2 = DiscountApplicator::new(
            campaign.discount_type.clone(),
            campaign.discount_amount,
            campaign.discount_message.clone(),
        );
        let looper2 = DiscountLoop::new(applicator2);
        
        looper2.loop_items(
            cart,
            &selector.group_b.items,
            selector.group_b.qty_needed * num_pairs,
        );
    }
}

// ================================ Rebuy Free Gift Logic ================================

fn apply_rebuy_free_gift(cart: &mut Cart, settings: &RebuyFreeGiftSettings) {
    let threshold_cents = settings.threshold_subtotal * 100;
    if cart.subtotal_price < threshold_cents {
        return;
    }

    for item in &mut cart.line_items {
        if item.line_price_changed {
            continue;
        }

        // Check if item has the attribution property
        if let Some(attr_value) = item.properties.get(&settings.rebuy_attribution_key) {
            if attr_value.to_lowercase() == settings.rebuy_attribution_value.to_lowercase()
                && settings.gift_variant_ids.contains(&item.variant_id)
            {
                // Apply 100% discount
                item.line_price = 0;
                item.line_price_changed = true;
                item.discount_message = Some(
                    settings
                        .free_gift_message
                        .replace("{{currency}}", &cart.presentment_currency),
                );
            }
        }
    }
}

// ================================ Main Processing ================================

fn process_cart(cart: &mut Cart) {
    // Run bundle discounts first
    let campaigns = get_grouped_bundle_discounts();
    run_grouped_bundle_discounts(cart, &campaigns);

    // Recalculate subtotal after bundle discounts
    cart.subtotal_price = cart.line_items.iter().map(|li| li.line_price).sum();

    // Apply rebuy free gift if enabled
    if ENABLE_REBUY_FREE_GIFT {
        let settings = get_rebuy_settings();
        apply_rebuy_free_gift(cart, &settings);
    }

    // Recalculate final subtotal
    cart.subtotal_price = cart.line_items.iter().map(|li| li.line_price).sum();
}