let run_example = false

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_char_occurences (s : string) (c_to_remove : char) : string =
  s |> String.to_seq |> Seq.filter (fun c -> c <> c_to_remove) |> String.of_seq

let read_input_file : string =
  let input_raw =
    read_file (if run_example then "example-input.txt" else "input.txt")
  in
  remove_char_occurences input_raw '\r'

let solve (input : string) : int =
  let repeated_num (start : int) : int =
    int_of_string (string_of_int start ^ string_of_int start)
  in
  let evaluate_section (section : string) : int =
    let split_end (s : string) : int * int =
      let s_s_s = String.sub s 0 (String.length s / 2)
      and s_e_s =
        String.sub s
          (String.length s / 2)
          (String.length s - (String.length s / 2))
      in

      ( (if String.length s_s_s = 0 then 0 else int_of_string s_s_s),
        if String.length s_e_s = 0 then 0 else int_of_string s_e_s )
    in
    let s, e =
      match section |> String.split_on_char '-' with
      | st :: en :: _ -> (st, en)
      | _ ->
          raise
            (Failure ("Section " ^ section ^ " has invalid format (missing -)"))
    in
    let s_s, _s_e = split_end s in
    let s = int_of_string s and e = int_of_string e in
    let rec find_sum (curr_start : int) (sum : int) : int =
      (*if curr_start > s_s && curr_start > e_s then sum
      else if curr_start = e_s then
        if
          curr_start |> string_of_int |> String.length
          <= (e_e |> string_of_int |> String.length)
          && (curr_start > s_s || curr_start >= s_e)
        then if curr_start <= e_e then sum + repeated_num curr_start else sum
        else sum
      else
        let new_sum =
          if curr_start = s_s then
            if curr_start >= s_e then sum + repeated_num curr_start else sum
          else if
            curr_start |> string_of_int |> String.length
            >= (s_e |> string_of_int |> String.length)
          then if curr_start < e_e then sum + repeated_num curr_start else sum
          else sum
        in
        find_sum (curr_start + 1) new_sum*)
      let curr_num = repeated_num curr_start in
      if curr_num > e then sum
      else
        find_sum (curr_start + 1)
          (if curr_num >= s then sum + curr_num else sum)
    in
    find_sum s_s 0
  in
  String.split_on_char ',' input
  |> List.map (fun s -> String.trim s)
  |> List.map (fun section -> evaluate_section section)
  |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let main () : int =
  let input = read_input_file in
  solve input

let () = print_endline (string_of_int (main ()))
