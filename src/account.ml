type result = Ok of int | Err of string

let string_of_result = function
  | Ok  v -> "Ok(" ^ (string_of_int v) ^ ")"
  | Err v -> "Err(" ^ v ^ ")"

let print_result r = print_endline (r |> string_of_result)

let make_withdraw balance =
  fun amount ->
    if !balance >= amount
    then (balance := !balance - amount;
          Ok !balance)
    else Err "Insufficient funds"

let () =
  let w1 = make_withdraw (ref 100)
  and w2 = make_withdraw (ref 100)
  in
  print_result (w1 10);
  print_result (w1 20);
  print_result (w1 80);
  
  print_result (w2 40)
