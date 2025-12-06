let run_example = false

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
    let rec inner ((f, s) as acc : int * int) (vals : int list) : int * int =
      match vals with
      | [] -> acc
      | h :: [] -> if h > s then (f, h) else acc
      | h :: t ->
          if h > f then inner (h, 0) t
          else if h > s then inner (f, h) t
          else inner acc t
    in
    let f, s =
      line |> String.to_seq
      |> Seq.map (fun c -> int_of_char c - int_of_char '0')
      |> List.of_seq
      |> inner (0, 0)
    in
    (f * 10) + s
  in
  input |> String.split_on_char '\n' |> List.map String.trim
  |> List.map solve_line
  |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let main () : int = read_input () |> solve
let () = print_endline (string_of_int (main ()))
