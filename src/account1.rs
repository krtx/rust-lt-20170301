fn main() {
    // fn id_i32() -> i32 -> i32{
    //     |x: i32| { x }
    // }

    // fn id_i32() -> Fn(i32) -> i32 {
    //     |x: i32| { x }
    // }

    // fn id_i32() -> Box<Fn(i32) -> i32> {
    //     Box::new(|x: i32| { x })
    // }

    // fn make_adder() -> Box<Fn(i32) -> i32> {
    //     let mut acc = 0;
    //     Box::new(|x: i32| {
    //         acc += x;
    //         acc
    //     })
    // }

    // fn make_adder() -> Box<FnMut(i32) -> i32> {
    //     let mut acc = 0;
    //     Box::new(move |x: i32| {
    //         acc += x;
    //         acc
    //     })
    // }

    fn make_adder(init: i32) -> Box<FnMut(i32) -> i32> {
        let mut acc = init;
        Box::new(move |x: i32| {
            acc += x;
            acc
        })
    }
    let mut adder = make_adder(10);
    println!("{:?}", adder(10));
}
