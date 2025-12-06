let run_example = false

type num_seq = int list

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input () : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let solve (input : string) : int =
  let solve_line (line : string) : int =
    let handle_new_value (v : int) (seq : num_seq) : num_seq =
      let rec inner (v : int) (rem : num_seq) (acc : num_seq) : num_seq =
        match rem with
        | [] -> List.rev acc
        | h :: t ->
            (* Definitely want to put a higher number at the front position -> put it there and try to insert the number that was there previously *)
            if v > h then inner h t (v :: acc)
              (* It is never beneficial to replace a front number with a lower one *)
            else if v < h then List.rev acc @ rem
            (* If they are equal, try to insert it to the next position *)
              else inner v t (h :: acc)
      in
      inner v seq []
    in
    let rec inner (acc : num_seq) (vals : int list) : num_seq =
      match vals with [] -> acc | h :: t -> inner (handle_new_value h acc) t
    in
    let nums =
      line |> String.to_seq
      |> Seq.map (fun c -> int_of_char c - int_of_char '0')
      |> List.of_seq
    in
    let initial = nums |> List.rev |> List.take 12 |> List.rev in
    inner initial (nums |> List.rev |> List.drop 12)
    |> List.fold_left (fun (acc : int) (v : int) -> (acc * 10) + v) 0
  in
  input |> String.split_on_char '\n' |> List.map String.trim
  |> List.map solve_line
  |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let main () : int = read_input () |> solve
let () = print_endline (string_of_int (main ()))
