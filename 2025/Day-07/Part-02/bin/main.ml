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

let add_positions_sorted (positions : (int * int) list) (data : int list) :
    int list =
  let rec inner (positions : (int * int) list) (data_rem : int list)
      (acc : int list) (i : int) : int list =
    match (positions, data_rem) with
    | [], _ -> List.rev acc @ data_rem
    | _, [] -> List.rev acc
    | (index, value) :: indices_left, curr_val :: rem ->
        if index = i then inner indices_left ((value + curr_val) :: rem) acc i
        else inner positions rem (curr_val :: acc) (i + 1)
  in
  inner positions data [] 0

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
    | h :: _ -> h |> List.map (fun _ -> 0)
    | _ -> raise (Failure "Invalid grid")
  in
  let final_rays =
    grid
    |> List.fold_left
         (fun (rays : int list) (row : field list) ->
           let row_with_i =
             row |> List.mapi (fun (i : int) (f : field) -> (i, f))
           in
           let rays_with_grid =
             List.map2
               (fun (rays_present : int) ((i, f) : int * field) ->
                 (rays_present, i, f))
               rays row_with_i
           in
           let new_ray_positions =
             rays_with_grid
             |> List.fold_left
                  (fun (new_positions : (int * int) list)
                       ((rays_present, i, f) : int * int * field) ->
                    if rays_present > 0 && f = Splitter then
                      (i - 1, rays_present)
                      :: (i + 1, rays_present)
                      :: new_positions
                    else if f = Source then (i, 1) :: new_positions
                    else new_positions)
                  []
           in
           let remove_positions =
             row_with_i
             |> List.fold_left
                  (fun (acc : int list) ((i, f) : int * field) ->
                    if f = Splitter then i :: acc else acc)
                  []
             |> List.sort ( - )
           in
           let new_ray_positions =
             new_ray_positions
             |> List.sort (fun ((i1, _) : int * int) ((i2, _) : int * int) ->
                    i1 - i2)
           in
           let new_rays =
             add_positions_sorted new_ray_positions
               (set_positions_sorted remove_positions rays 0)
           in
           let _ =
             new_rays
             |> List.map (fun v ->
                    let _ = v |> print_int in
                    " " |> print_string)
           in
           let _ = "\r\n" |> print_endline in
           new_rays)
         initial_rays
  in
  final_rays |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let main () : int = run_example |> read_input |> solve
let () = main () |> string_of_int |> print_endline
