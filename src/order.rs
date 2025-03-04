#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
    Stop,
    GoodTillCanceled,
    FillOrKillLimit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Order {
    id: usize,
    order_type: OrderType,
    side: Side,
    price: f64,
    quantity: u32,
}

impl Order {
    pub fn new(id: usize, order_type: OrderType, side: Side, price: f64, quantity: u32) -> Self {
        Self {
            id,
            order_type,
            side,
            price,
            quantity,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn order_type(&self) -> OrderType {
        self.order_type
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn price(&self) -> f64 {
        self.price
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }

    // update order quantity
    pub fn update_quantity(&mut self, quantity: u32) {
        self.quantity = quantity;
    }
}
