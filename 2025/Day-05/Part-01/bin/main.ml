let run_example = false

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let parse_range (s : string) : int * int =
  match s |> String.split_on_char '-' with
  | [ f; s ] -> (int_of_string f, int_of_string s)
  | _ -> raise (Failure ("Invalid range '" ^ s ^ "'"))

let solve (input : string) : int =
  match input |> Str.split (Str.regexp "\n\n") |> List.map String.trim with
  | [ fresh_ranges_input; ingredients_input ] ->
      let fresh_ranges =
        fresh_ranges_input |> String.split_on_char '\n' |> List.map String.trim
        |> List.map parse_range
      in
      let ingredients =
        ingredients_input |> String.split_on_char '\n' |> List.map String.trim
        |> List.map int_of_string
      in
      ingredients
      |> List.map (fun (i : int) ->
             if
               List.exists
                 (fun ((s, e) : int * int) -> s <= i && i <= e)
                 fresh_ranges
             then 1
             else 0)
      |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0
  | _ -> raise (Failure "Invalid input")

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
