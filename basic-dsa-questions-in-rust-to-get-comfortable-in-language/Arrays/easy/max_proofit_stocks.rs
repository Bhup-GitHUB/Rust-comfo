//Imagining youre are buying stocks  throughout the year and you want to maximize your profit

//stock prices-[7,1,5,3,6,4]

// find the diffrence between the max and min price

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut min_price = prices[0];
    let mut max_profit = 0;

    for &price in prices.iter() {
        min_price = min_price.min(price);
        let profit = price - min_price;
        max_profit = max_profit.max(profit);
    }

    println!("{}", max_profit);
    max_profit
}

fn main() {
    let prices = vec![7, 1, 5, 3, 6, 4];
    max_profit(prices);
}
