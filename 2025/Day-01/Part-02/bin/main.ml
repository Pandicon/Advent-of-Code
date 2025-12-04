let run_example = false

let read_file (file_name : string) : string =
  In_channel.with_open_bin file_name In_channel.input_all

let remove_char_occurences (s : string) (c_to_remove : char) : string =
  s |> String.to_seq |> Seq.filter (fun c -> c <> c_to_remove) |> String.of_seq

let read_input_full_file : string =
  let input_raw =
    read_file (if run_example then "example-input.txt" else "input.txt")
  in
  remove_char_occurences input_raw '\r'

let modulo (x : int) (n : int) : int =
  let rem = x mod n in
  if rem >= 0 then rem else rem + n

let solve (input : string) : int =
  let modulo_n = 100 and start_pos = 50 in
  let lines = String.split_on_char '\n' input in
  let rotate (position : int) (delta : int) (score : int) : int * int =
    (* However many 100s fit into the change is how many times the dial goes through zero. And the position after these rotations is unchanged, so we can ignore them from now on. *)
    let cycles = abs delta / modulo_n
    and new_position = modulo (position + delta) modulo_n
    and to_add = delta mod modulo_n in
    (* If we end at a zero after the remaining rotation, we add 1 *)
    let score_to_add = if new_position = 0 then cycles + 1 else cycles in
    (* If we went over 100 with the remaining bit of rotation (so for example 80 -> 120 with R40) or went below 0 but did not start at zero (so had to cross it from a positive number), add 1
    Since the remaining rotation is always > -100 and < 100 and the initial position is >= 0 and < 100 it is not possible to cross -100 or 200 *)
    let score_to_add =
      if (position <> 0 && position + to_add < 0) || position + to_add > 100
      then score_to_add + 1
      else score_to_add
    in
    (new_position, score + score_to_add)
  in
  let rec inner (rows_to_handle : string list) (position : int) (score : int) :
      int =
    match rows_to_handle with
    | [] -> score
    | row :: rest ->
        let direction = String.sub row 0 1
        and delta = int_of_string (String.sub row 1 (String.length row - 1)) in
        let delta = if direction = "L" then -delta else delta in
        let new_pos, new_score = rotate position delta score in
        inner rest new_pos new_score
  in
  inner lines start_pos 0

let main () : int =
  let input = read_input_full_file in
  solve input

let () = print_endline (string_of_int (main ()))
