use crate::order::{Order, Side};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct OrderBook {
    orders: HashMap<usize, Order>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            orders: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.insert(order.id(), order);
    }

    pub fn cancel_order(&mut self, id: usize) {
        self.orders.remove(&id);
    }

    pub fn match_orders(&mut self) {
        let mut matches: Vec<(usize, usize, u32)> = Vec::new();
        let orders: Vec<_> = self.orders.values().cloned().collect();
        
        // Find all matches without holding any borrows
        for order in &orders {
            if let Some(match_id) = self.find_match(order, order.quantity(), true) {
                matches.push((order.id(), *match_id, order.quantity()));
            }
        }
        
        // Process matches
        for (order_id, match_id, quantity) in matches {
            if let Some(matched_order) = self.orders.get_mut(&match_id) {
                println!(
                    "Matched Order ID: {} with Order ID: {} at Price: {} quantity: {}",
                    order_id,
                    matched_order.id(),
                    matched_order.price(),
                    quantity
                );
                
                matched_order.update_quantity(matched_order.quantity().saturating_sub(quantity));
                if matched_order.quantity() == 0 {
                    self.orders.remove(&match_id);
                }
            }
        }
    }

    pub fn print_orders(&self) {
        for order in &self.orders {
            println!(
                "Order ID: {}, Price: {}, Quantity: {}, Side: {:?}",
                order.1.id(),
                order.1.price(),
                order.1.quantity(),
                order.1.side()
            );
        }
    }

    // Find a match for a given order
    fn find_match(&self, order: &Order, quantity: u32, full_match: bool) -> Option<&usize> {
        self.orders
            .iter()
            .find(|o| {
                o.1.side() != order.side()
                    && (order.side() == Side::Buy && o.1.price() <= order.price()
                        || order.side() == Side::Sell && o.1.price() >= order.price())
                    && (!full_match || order.quantity() >= quantity)
            })
            .map(|o| o.0)
    }
}
