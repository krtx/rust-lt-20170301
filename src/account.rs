fn main() {
    type Account = Box<FnMut(i32) -> Result<i32, &'static str>>;

    fn make_withdraw(mut balance: i32) -> Account {
        Box::new(move |amount: i32| {
            if balance >= amount {
                balance -= amount;
                Ok(balance)
            } else {
                Err("Insufficient funds")
            }
        })
    }

    let mut w1 = make_withdraw(100);
    let mut w2 = make_withdraw(100);

    println!("{:?}", w1(10));
    println!("{:?}", w1(20));
    println!("{:?}", w1(80));

    println!("{:?}", w2(40));
}
