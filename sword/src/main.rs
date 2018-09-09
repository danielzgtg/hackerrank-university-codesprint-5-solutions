use std::io::{/*self,*/ BufRead};

const MODULO: i64 = 1000000007;

struct Shop {
    value: i64,
    baserate: i64,
    perrate: i64,
    transactionfee: i64,
    depreciation: i64,
}

impl Shop {
    fn new(value: i64, baserate: i64, perrate: i64, transactionfee: i64, depreciation: i64) -> Shop {
        Shop {
            value,
            baserate,
            perrate,
            transactionfee,
            depreciation,
        }
    }
}

fn best_value(shops: &[Shop], begin: usize) -> i64 {
    let here = &shops[begin];
    let depreciation = here.depreciation;
    let mut value = here.value;
    let mut high_revenue = value - here.transactionfee;
    for i in begin+1..shops.len() {
        value -= depreciation;
        if value < 0 {
            return high_revenue
        }
        let shop = &shops[i];
        let new_revenue = value - shop.transactionfee;
        if new_revenue > high_revenue {
            high_revenue = new_revenue;
        }
    }
    high_revenue
}

fn solve(shops: &[Shop]) -> i64{
    let mut profit: i64 = 0;
    for i in 0..shops.len() {
        let bestvalue = best_value(shops, i);
        let me = &shops[i];
        let perrate = me.perrate;
        let mut cost = me.baserate + perrate;
        let mut transaction = bestvalue - cost;
        while transaction > 0 {
            profit = profit + transaction;
            cost += perrate;
            transaction = bestvalue - cost;
        }
    }
    profit % MODULO
}

fn main() {
    let shops: Vec<Shop>;
    {
        let mut read = String::new();
        //let stdin_ = io::stdin();
        //let mut stdin = stdin_.lock();
        let mut stdin = "5
158 5 8 148 10
200 30 7 158 20
100 16 10 138 15
200 40 30 115 12
300 160 50 100 5".as_bytes();
        stdin.read_line(&mut read).unwrap();
        let count: usize = read.trim().parse().unwrap();
        let mut tmpshops = Vec::with_capacity(count);
        for _ in 0..count {
            read.clear();
            stdin.read_line(&mut read).unwrap();
            let mut iter = read.trim().split_whitespace();
            let value: i64 = iter.next().unwrap().parse().unwrap();
            let baserate: i64 = iter.next().unwrap().parse().unwrap();
            let perrate: i64 = iter.next().unwrap().parse().unwrap();
            let transactionfee: i64 = iter.next().unwrap().parse().unwrap();
            let depreciation: i64 = iter.next().unwrap().parse().unwrap();
            let shop = Shop::new(value, baserate, perrate, transactionfee, depreciation);
            tmpshops.push(shop);
        }
        shops = tmpshops;
    }
    for _ in 0..10000 {
        solve(&shops);
    }
    //println!("{}", solve(&shops));
}
