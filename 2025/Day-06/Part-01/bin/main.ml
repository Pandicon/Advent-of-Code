let run_example = false

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let solve (input : string) : int =
  let rows =
    input |> String.split_on_char '\n'
    |> List.map (fun (line : string) ->
           line |> String.split_on_char ' '
           |> List.filter (fun (s : string) -> s <> ""))
    |> List.rev
  in
  match rows with
  | [] -> raise (Failure "Invalid input")
  | operations :: problems ->
      let problem_numbers =
        problems
        |> List.fold_left
             (fun (acc : int list list) (v : string list) ->
               List.map2
                 (fun (problem : int list) (num : string) ->
                   int_of_string num :: problem)
                 acc v)
             (operations |> List.map (fun (_v : string) -> []))
      in
      List.map2
        (fun (operation : string) (problem_nums : int list) ->
          match operation with
          | "*" ->
              List.fold_left
                (fun (acc : int) (v : int) -> acc * v)
                1 problem_nums
          | "+" ->
              List.fold_left
                (fun (acc : int) (v : int) -> acc + v)
                0 problem_nums
          | _ -> raise (Failure ("Invalid operation '" ^ operation ^ "'")))
        operations problem_numbers
      |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
