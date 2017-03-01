fn make_withdraw(mut balance: i32) ->
    Box<FnMut(i32) -> Result<i32, &'static str>> {
    Box::new(move |amount: i32| {
        if balance >= amount {
            balance -= amount;
            Ok(balance)
        } else { Err("Insufficient funds") }
    })
}
let mut w1 = make_withdraw(100);
let mut w2 = make_withdraw(100);

println!("{:?}", w1(10)); // Ok(90)
println!("{:?}", w1(20)); // Ok(70)
println!("{:?}", w1(80)); // Err("Insufficient funds")

println!("{:?}", w2(40)); // Ok(60)
