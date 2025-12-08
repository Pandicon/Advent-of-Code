let run_example = false

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c_to_remove : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun c -> c <> c_to_remove) |> String.of_seq

let read_input () : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let solve (input : string) : int =
  let unwrap_or_value (value : 'a) (o : 'a option) : 'a =
    match o with Option.Some v -> v | Option.None -> value
  in
  let create_indices (row_i : int) (col_i : int) : (int * int) list =
    [
      (row_i - 1, col_i - 1);
      (row_i, col_i - 1);
      (row_i + 1, col_i - 1);
      (row_i - 1, col_i);
      (row_i + 1, col_i);
      (row_i - 1, col_i + 1);
      (row_i, col_i + 1);
      (row_i + 1, col_i + 1);
    ]
  in
  let grid =
    input |> String.split_on_char '\n'
    |> List.map (fun line ->
           line |> String.trim |> String.to_seq |> List.of_seq
           |> List.map (fun c ->
                  if c = '.' then 0
                  else if c = '@' then 1
                  else
                    raise
                      (Failure ("Invalid character '" ^ String.make 1 c ^ "'"))))
  in
  let grid_arr =
    grid
    |> List.map (fun (line : int list) -> FunctionalArray.from_list line)
    |> FunctionalArray.from_list
  in
  grid
  |> List.mapi (fun (rowi : int) (row : int list) ->
         row
         |> List.mapi (fun (coli : int) (v : int) ->
                if v = 0 then -1
                else
                  create_indices rowi coli
                  |> List.map (fun (ri, ci) ->
                         match FunctionalArray.get ri grid_arr with
                         | Option.None -> 0
                         | Option.Some r ->
                             FunctionalArray.get ci r |> unwrap_or_value 0)
                  |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0)
         |> List.map (fun (v : int) -> if 0 <= v && v < 4 then 1 else 0))
  |> List.map (fun (l : int list) ->
         l |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0)
  |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let main () : int = read_input () |> solve
let () = print_endline (string_of_int (main ()))
