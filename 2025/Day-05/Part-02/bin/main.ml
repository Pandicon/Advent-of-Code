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
  | [ fresh_ranges_input; _ingredients_input ] -> (
      let fresh_ranges =
        fresh_ranges_input |> String.split_on_char '\n' |> List.map String.trim
        |> List.map parse_range
        |> List.sort (fun ((s1, _e1) : int * int) ((s2, _e2) : int * int) ->
               s1 - s2)
      in
      let rec build_merged_ranges (ranges_left : (int * int) list)
          (acc : (int * int) list) ((s, e) as current_range : int * int) :
          (int * int) list =
        match ranges_left with
        | [] -> current_range :: acc
        | ((hs, he) as h) :: t ->
            if e < hs then build_merged_ranges t (current_range :: acc) h
            else if s > he then build_merged_ranges t (h :: acc) current_range
            else build_merged_ranges t acc (min s hs, max e he)
      in
      match fresh_ranges with
      | [] -> 0
      | h :: t ->
          let merged_ranges = build_merged_ranges t [] h in
          merged_ranges
          |> List.map (fun ((s, e) : int * int) -> e - s + 1)
          |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0)
  | _ -> raise (Failure "Invalid input")

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
