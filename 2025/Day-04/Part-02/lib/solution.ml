let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let shift_list_left (l : 'a list) : 'a list =
  match l with [] -> [] | h :: t -> t @ [ h ]

let shift_list_right (l : 'a list) : 'a list =
  let rec inner (l : 'a list) (acc : 'a list) : 'a list =
    match l with
    | [] -> List.rev acc
    | h :: [] -> h :: List.rev acc
    | h :: t -> inner t (h :: acc)
  in
  inner l []

let rec shift_list_left_amount (amount : int) (l : 'a list) =
  if amount = 0 then l
  else if amount > 0 then
    shift_list_left_amount (amount - 1) (l |> shift_list_left)
  else shift_list_left_amount (amount + 1) (l |> shift_list_right)

let apply_f_to_elements_of_grids (base_grid : 'a list list) (f : 'a -> 'a -> 'a)
    (grids : 'a list list list) : 'a list list =
  grids
  |> List.fold_left
       (fun (acc : 'a list list) (gr : 'a list list) ->
         acc
         |> List.map2
              (fun (acc_l : 'a list) (gr_l : 'a list) ->
                acc_l
                |> List.map2 (fun (acc_v : 'a) (gr_v : 'a) -> f acc_v gr_v) gr_l)
              gr)
       base_grid

let list_singleton (v : 'a) : 'a list = [ v ]

let count_empty (grid : int list list) : int =
  grid
  |> List.map
       (List.fold_left
          (fun (acc : int) (v : int) -> if v = 0 then acc + 1 else acc)
          0)
  |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let pad_grid (grid : int list list) : int list list =
  match grid with
  | [] -> raise (Failure "Grid must not be empty to be padded")
  | h :: _ ->
      let new_row = 0 :: 0 :: (h |> List.map (fun (_ : int) -> 0)) in
      new_row
      :: ((grid |> List.map (fun (row : int list) -> 0 :: (row @ [ 0 ])))
         @ [ new_row ])

let solve (input : string) : int =
  let shifts =
    [ (-1, -1); (0, -1); (1, -1); (-1, 0); (1, 0); (-1, 1); (0, 1); (1, 1) ]
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
    |> pad_grid
  in
  let initial_empty = grid |> count_empty in
  let zero_grid =
    grid |> List.map (fun (r : int list) -> r |> List.map (fun _ -> 0))
  in
  let rec inner (grid : int list list) (prev_empty : int) : int =
    let grid_neighbours =
      shifts
      |> List.map (fun ((hor, ver) : int * int) : int list list ->
             grid |> shift_list_left_amount hor
             |> List.map (shift_list_left_amount ver))
      |> apply_f_to_elements_of_grids zero_grid (fun (a : int) (b : int) ->
             a + b)
      |> list_singleton
      |> apply_f_to_elements_of_grids grid (fun (a : int) (b : int) -> a * b)
    in
    let new_grid =
      grid_neighbours
      |> List.map (List.map (fun (v : int) -> if v < 4 then 0 else 1))
    in
    let new_empty = new_grid |> count_empty in
    if new_empty = prev_empty then new_empty else inner new_grid new_empty
  in
  let final_empty = inner grid initial_empty in
  final_empty - initial_empty
