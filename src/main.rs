type InstrumentID = u64;
type Time = u64;
type Volume = u64;
type Price = u64;

enum Side {
    Ask,
    Bid
}
impl Side {
    fn side_sign(&self) -> i64{
        match self {
            &Self::Ask => {-1}
            &Self::Bid => { 1}
        }
    }

    fn side_idx(&self) -> usize {
        match self {
            &Self::Ask => {1},
            &Self::Bid => {0}
        }
    }
}
#[derive(Clone, Default)]
struct QuoteSide {
    time: Time,
    volume: Volume
   
}

trait Quote {
    fn get_volume_at(&self, side: &Side) -> Volume;
    fn get_volume_total(&self) -> Volume;
    fn contains_side(&self, side : &Side) -> bool; 
} 
#[derive(Clone, Default)]
struct QuoteL2 {
    orders: [QuoteSide;2]
}

impl Quote for QuoteL2 {
    fn get_volume_at(&self, side: &Side) -> Volume {
        match *side {
            Side::Ask => {self.orders[1].volume},
            Side::Bid => {self.orders[0].volume}
        }
    }
    fn get_volume_total(&self) -> Volume {
        self.orders[0].volume + self.orders[1].volume
    }
    fn contains_side(&self, side : &Side) -> bool {
        match side {
            Side::Ask => {
                return self.orders[1].volume != 0;
            },
            Side::Bid => {
                return self.orders[0].volume != 0;
            }
        }
    }
}
#[derive(Clone, Default)]
struct DenseQuotes {
    quotes: Vec<QuoteL2>,
    empty_quote: QuoteL2
}

trait Quotes<T : Quote> {
    fn get(&self, price : Price) -> &T;
    fn get_or_create(&mut self, price: &Price) -> &T;
    fn next_nonzero_quote(&self, side: &Side, price: &Price) -> Price;
    fn prev_nonzero_quote(&self, side: &Side, price: &Price) -> Price;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

impl Quotes<QuoteL2> for DenseQuotes {
    fn get(&self, price : Price) -> &QuoteL2 {
        if price as usize >= self.quotes.len() || price <= 0 {
            return &self.empty_quote;
        }
        return &self.quotes[price as usize];
    }
    fn get_or_create(&mut self, price: &Price) -> &QuoteL2 {
        if *price < 0 {
            panic!("negative price is incompatible with DenseQuotes");
        }
        if (*price as usize >= self.quotes.len()) {
            self.quotes.resize(*price as usize + 128 , QuoteL2::default());
        }
        return &self.quotes[*price as usize] ;
    }
    fn next_nonzero_quote(&self, side: &Side,  price: &Price) -> Price {
        let mut px: Price = *price;
        while (px as usize) < self.quotes.len() && !self.quotes[px as usize].contains_side(side) {
            px += 1;
        }
        return if px as usize >= self.quotes.len()  {0} else {px}
    }
    fn prev_nonzero_quote(&self, side: &Side, price: &Price) -> Price {
        let mut px: Price = *price;
        if (px as usize >= self.quotes.len()) {
            px = self.quotes.len() as u64 - 1;
        }
        while px > 0 && self.quotes[px as usize].contains_side(side) {
            px -= 1;
        }
        return px;
    }
    fn is_empty(&self) -> bool {
        self.quotes.is_empty()
    }
    fn clear(&mut self) {
        self.quotes.clear()
    }
}

#[derive(Clone, Default)]
struct OrderBookL2 {
    isin_id : InstrumentID,
    quotes: Box<DenseQuotes>,
}

struct UpdateL2 {}

trait OrderBook {
    fn update(upd : &UpdateL2);
    fn get_volume_at_px(price : &Price) -> Volume;
    fn get_volume_at_side_px(&self, side: &Side, price: &Price) -> Volume;
    fn get_best_price(&self, side: &Side) -> Price;
    fn get_price_by_index(&self, side :&Side, index : i64) -> Price;

}

impl OrderBook for OrderBookL2 {
    fn update(upd : &UpdateL2) {unimplemented!()}
    fn get_volume_at_px(price : &Price) -> Volume {unimplemented!()}
    fn get_volume_at_side_px(&self, side: &Side, price: &Price) -> Volume {unimplemented!()}
    fn get_best_price(&self, side: &Side) -> Price {unimplemented!()}
    fn get_price_by_index(&self, side :&Side, index : i64) -> Price{
        return self.get_best_price(side) - (side.side_sign() * index) as Price;
    }
}


fn main() {
    
}

