let run_example = false

type field = Space | Splitter | Source

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let set_positions_sorted (positions : int list) (data : 'a list) (value : 'a) :
    'a list =
  let rec inner (positions : int list) (data_rem : 'a list) (acc : 'a list)
      (i : int) : 'a list =
    match (positions, data_rem) with
    | [], _ -> List.rev acc @ data_rem
    | _, [] -> List.rev acc
    | index :: indices_left, curr_val :: rem ->
        if index = i then inner indices_left rem (value :: acc) (i + 1)
        else inner positions rem (curr_val :: acc) (i + 1)
  in
  inner positions data [] 0

let solve (input : string) : int =
  let grid =
    input |> String.split_on_char '\n' |> List.map String.trim
    |> List.map (fun (l : string) -> l |> String.to_seq |> List.of_seq)
    |> List.map (fun (l : char list) ->
           l
           |> List.map (fun (c : char) ->
                  match c with
                  | '.' -> Space
                  | '^' -> Splitter
                  | 'S' -> Source
                  | _ ->
                      raise
                        (Failure
                           ("Unexpected character '" ^ String.make 1 c ^ "'"))))
  in
  let initial_rays =
    match grid with
    | h :: _ -> h |> List.map (fun _ -> false)
    | _ -> raise (Failure "Invalid grid")
  in
  let _, splits =
    grid
    |> List.fold_left
         (fun ((rays, splits_so_far) : bool list * int) (row : field list) ->
           let row_with_i =
             row |> List.mapi (fun (i : int) (f : field) -> (i, f))
           in
           let rays_with_grid =
             List.map2
               (fun (ray_present : bool) ((i, f) : int * field) ->
                 (ray_present, i, f))
               rays row_with_i
           in
           let new_splits, new_ray_positions =
             rays_with_grid
             |> List.fold_left
                  (fun ((splits, new_positions) : int * int list)
                       ((ray_present, i, f) : bool * int * field) ->
                    if ray_present && f = Splitter then
                      (splits + 1, (i - 1) :: (i + 1) :: new_positions)
                    else if f = Source then (splits, i :: new_positions)
                    else (splits, new_positions))
                  (0, [])
           in
           let remove_positions =
             row_with_i
             |> List.fold_left
                  (fun (acc : int list) ((i, f) : int * field) ->
                    if f = Splitter then i :: acc else acc)
                  []
             |> List.sort ( - )
           in
           let new_ray_positions = new_ray_positions |> List.sort_uniq ( - ) in
           let new_rays =
             set_positions_sorted new_ray_positions
               (set_positions_sorted remove_positions rays false)
               true
           in
           (*let a =
             new_rays
             |> List.fold_left
                  (fun (acc : int) (v : bool) -> if v then acc + 1 else acc)
                  0
           in
           let _ = a |> string_of_int |> print_endline in*)
           (new_rays, splits_so_far + new_splits))
         (initial_rays, 0)
  in
  splits

let main () : int = run_example |> read_input |> solve
let () = main () |> string_of_int |> print_endline
