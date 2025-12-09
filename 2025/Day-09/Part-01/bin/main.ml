let run_example = false

type position = int * int

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let parse_line (line : string) : position =
  match line |> String.split_on_char ',' with
  | [ f; s ] -> (int_of_string f, int_of_string s)
  | _ -> raise (Failure ("Line '" ^ line ^ "' is of invalid format"))

let calculate_area ((x1, y1) : position) ((x2, y2) : position) : int =
  (abs (x1 - x2) + 1) * (abs (y1 - y2) + 1)

let solve (input : string) : int =
  let points =
    input |> String.split_on_char '\n' |> List.map String.trim
    |> List.map parse_line
  in
  let areas =
    points
    |> List.map (fun (point : position) ->
           points
           |> List.map (fun (point2 : position) -> calculate_area point point2))
    |> List.flatten
    |> List.sort (fun (area1 : int) (area2 : int) -> area2 - area1)
  in
  match areas with h :: _ -> h | [] -> raise (Failure "No areas found")

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
