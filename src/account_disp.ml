let make_withdraw
    (balance : int ref) : int -> result =
  fun amount ->
    if !balance >= amount
    then (balance := !balance - amount;
          Ok !balance)
    else Err "Insufficient funds"

let () =
  let w1 = make_withdraw (ref 100)
  and w2 = make_withdraw (ref 100)
  in
  print_result (w1 10); (* Ok(90) *)
  print_result (w1 20); (* Ok(70) *)
  print_result (w1 80); (* Err("Insufficient funds") *)
  
  print_result (w2 40)  (* Ok(60) *)
