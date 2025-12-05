let run_example = false

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c_to_remove : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun c -> c <> c_to_remove) |> String.of_seq

let read_input_file : string =
  read_file (if run_example then "example-input.txt" else "input.txt")
  |> remove_character_occurences '\r'

let solve (input : string) : int =
  let repeated_num (start : int) : int =
    int_of_string (string_of_int start ^ string_of_int start)
  in
  let evaluate_section (section : string) : int =
    let s, e =
      match section |> String.split_on_char '-' with
      | st :: en :: _ -> (st, en)
      | _ ->
          raise
            (Failure ("Section " ^ section ^ " has invalid format (missing -)"))
    in
    let s = int_of_string s and e = int_of_string e in
    let rec find_values (curr_start : int) (values : int list) : int list =
      let curr_num = repeated_num curr_start
      and curr_start_s = string_of_int curr_start in
      if curr_num > e then values
      else
        let rec repeat_concat (last : string) (acc : int list) : int list =
          let current_s = curr_start_s ^ last in
          let curr = int_of_string current_s in
          if curr > e then acc
          else repeat_concat current_s (if curr >= s then curr :: acc else acc)
        in
        let new_values = repeat_concat "" values in
        find_values (curr_start + 1) new_values
    in
    find_values 1 []
    |> List.sort_uniq (fun (a : int) (b : int) -> a - b)
    |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0
  in
  String.split_on_char ',' input
  |> List.map (fun s -> String.trim s)
  |> List.map (fun section -> evaluate_section section)
  |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let main () : int =
  let input = read_input_file in
  solve input

let () = print_endline (string_of_int (main ()))
