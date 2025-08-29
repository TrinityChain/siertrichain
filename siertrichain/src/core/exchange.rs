use crate::core::triangle::Triangle;
use crate::wallet::address::Address;
use crate::core::hash::H256;
use rust_decimal::Decimal;
use std::collections::HashMap;

/// Represents a buy or sell order for a triangular region.
pub enum OrderType {
    Buy,
    Sell,
}

/// Represents an order in the exchange.
pub struct Order {
    /// The address of the trader.
    pub trader: Address,
    /// The type of the order (buy or sell).
    pub order_type: OrderType,
    /// The triangle being traded.
    pub triangle: Triangle,
    /// The price of the order.
    pub price: Decimal,
}

/// A simple order book for a single triangular region.
pub struct OrderBook {
    /// A list of buy orders.
    pub buy_orders: Vec<Order>,
    /// A list of sell orders.
    pub sell_orders: Vec<Order>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            buy_orders: Vec::new(),
            sell_orders: Vec::new(),
        }
    }
}

/// Manages the decentralized exchange for triangular regions.
pub struct Exchange {
    /// A map from a triangle identifier to its order book.
    order_books: HashMap<H256, OrderBook>,
}

impl Exchange {
    pub fn new() -> Self {
        Self {
            order_books: HashMap::new(),
        }
    }

    /// Places a new order in the exchange.
    pub fn place_order(&mut self, order: Order) {
        let triangle_id = order.triangle.hash();
        let order_book = self.order_books.entry(triangle_id).or_insert_with(OrderBook::new);

        match order.order_type {
            OrderType::Buy => {
                // In a real implementation, we would match against sell orders.
                order_book.buy_orders.push(order);
                order_book.buy_orders.sort_by(|a, b| b.price.cmp(&a.price)); // Highest price first
            }
            OrderType::Sell => {
                // In a real implementation, we would match against buy orders.
                order_book.sell_orders.push(order);
                order_book.sell_orders.sort_by(|a, b| a.price.cmp(&b.price)); // Lowest price first
            }
        }
    }
}
