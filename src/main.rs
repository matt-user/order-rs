use order_rs::order::*;
use order_rs::order_book::*;

fn main() {
    let mut order_book = OrderBook::new();

    let order1 = Order::new(1, OrderType::Market, Side::Buy, 0.0, 10);
    let order2 = Order::new(2, OrderType::Limit, Side::Sell, 101.0, 20);
    let order3 = Order::new(3, OrderType::Limit, Side::Sell, 99.0, 5);
    let order4 = Order::new(4, OrderType::Market, Side::Sell, 0.0, 15);
    let order5 = Order::new(5, OrderType::GoodTillCanceled, Side::Buy, 102.0, 10);
    let order6 = Order::new(6, OrderType::FillOrKillLimit, Side::Sell, 100.0, 8);
    let order7 = Order::new(7, OrderType::FillOrKillLimit, Side::Buy, 99.0, 12);
    let order8 = Order::new(8, OrderType::FillOrKillLimit, Side::Buy, 101.0, 8);

    // Add orders to the order book
    order_book.add_order(order1);
    order_book.add_order(order2);
    order_book.add_order(order3);
    order_book.add_order(order4);
    order_book.add_order(order5);
    order_book.add_order(order6);
    order_book.add_order(order7);
    order_book.add_order(order8);

    // Print the order book before matching orders
    println!("Order Book before matching:");
    order_book.print_orders();

    // Match orders in the order book
    order_book.match_orders();

    // Print the order book after matching
    println!("Order Book after matching:");
    order_book.print_orders();
}
