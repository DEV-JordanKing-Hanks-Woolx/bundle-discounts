fn main() {
    let mut cart = Cart {
        line_items: vec![
            LineItem {
                id: 1,
                variant_id: 123,
                product_id: 7328618152002,
                quantity: 1,
                line_price: 5000, // $50.00
                line_price_changed: false,
                properties: HashMap::new(),
                discount_message: None,
            },
            LineItem {
                id: 2,
                variant_id: 456,
                product_id: 7328623067202,
                quantity: 1,
                line_price: 6000, // $60.00
                line_price_changed: false,
                properties: HashMap::new(),
                discount_message: None,
            },
        ],
        subtotal_price: 11000,
        presentment_currency: "USD".to_string(),
    };

    println!("Before processing:");
    println!("Subtotal: ${}.{:02}", cart.subtotal_price / 100, cart.subtotal_price % 100);

    process_cart(&mut cart);

    println!("\nAfter processing:");
    println!("Subtotal: ${}.{:02}", cart.subtotal_price / 100, cart.subtotal_price % 100);
    for (idx, item) in cart.line_items.iter().enumerate() {
        println!(
            "Item {}: Price ${}.{:02} - {}",
            idx + 1,
            item.line_price / 100,
            item.line_price % 100,
            item.discount_message.as_ref().unwrap_or(&"No discount".to_string())
        );
    }
}
