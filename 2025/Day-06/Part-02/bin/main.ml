let run_example = false

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let pad_to_len (l : 'a list) (len : int) (v : 'a) : 'a list =
  l @ List.init (len - List.length l) (fun (_i : int) -> v)

let rec split_into_problems (acc : int list list) (curr : int list)
    (remaining : int list) : int list list =
  match remaining with
  | [] -> List.rev (curr :: acc)
  | h :: t ->
      if h = 0 then split_into_problems (curr :: acc) [] t
      else split_into_problems acc (h :: curr) t

let solve (input : string) : int =
  let rows =
    input |> String.split_on_char '\n'
    |> List.map (fun (line : string) -> line |> String.to_seq |> List.of_seq)
    |> List.rev
  in
  match rows with
  | [] -> raise (Failure "Invalid input")
  | operations :: problems_rows ->
      let operations = operations |> List.filter (fun (c : char) -> c <> ' ') in
      let problems_rows =
        problems_rows |> List.rev
        |> List.map (fun (line : char list) ->
               line
               |> List.map (fun (c : char) ->
                      if c = ' ' then -1 else int_of_char c - int_of_char '0'))
      in
      let row_length =
        problems_rows |> List.map List.length
        |> List.fold_left (fun (acc : int) (v : int) -> max acc v) 0
      in
      let problem_numbers =
        problems_rows
        |> List.fold_left
             (fun (columns : int list list) (row : int list) ->
               List.map2
                 (fun (column : int list) (v : int) -> v :: column)
                 columns row)
             (pad_to_len [] row_length [])
        |> List.map (fun (column : int list) ->
               column |> List.rev
               |> List.fold_left
                    (fun (acc : int) (v : int) ->
                      if v = -1 then acc else (acc * 10) + v)
                    0)
        |> split_into_problems [] []
      in
      List.map2
        (fun (operation : char) (problem_nums : int list) ->
          match operation with
          | '*' ->
              List.fold_left
                (fun (acc : int) (v : int) -> acc * v)
                1 problem_nums
          | '+' ->
              List.fold_left
                (fun (acc : int) (v : int) -> acc + v)
                0 problem_nums
          | _ ->
              raise
                (Failure ("Invalid operation '" ^ String.make 1 operation ^ "'")))
        operations problem_numbers
      |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
