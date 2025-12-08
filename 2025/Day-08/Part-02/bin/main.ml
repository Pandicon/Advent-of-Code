let run_example = false

type position = int * int * int

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let parse_line (line : string) : position =
  match String.split_on_char ',' line with
  | [ f; s; t ] -> (int_of_string f, int_of_string s, int_of_string t)
  | _ -> raise (Failure ("The line " ^ line ^ " is of invalid format"))

let distance_squared ((x1, y1, z1) : position) ((x2, y2, z2) : position) : int =
  ((x1 - x2) * (x1 - x2)) + ((y1 - y2) * (y1 - y2)) + ((z1 - z2) * (z1 - z2))

let solve (input : string) : int =
  let positions =
    input |> String.split_on_char '\n' |> List.map String.trim
    |> List.map parse_line
  in
  let distances =
    positions
    |> List.map (fun (pos1 : position) ->
           positions
           |> List.map (fun (pos2 : position) ->
                  (pos1, pos2, pos2 |> distance_squared pos1)))
    |> List.flatten
    |> List.filter
         (fun ((_pos1 : position), (_pos2 : position), (distance : int)) ->
           distance <> 0)
    |> List.sort_uniq (fun (_pos11, _pos12, dist1) (_pos21, _pos22, dist2) ->
           dist1 - dist2)
  in
  let rec merge ((x1, y1, z1) as pos1 : position)
      ((x2, y2, z2) as pos2 : position) (initial : position list)
      (connections_left : position list list) (acc : position list list) :
      position list list =
    match connections_left with
    | [] -> initial :: acc
    | h :: t ->
        if
          List.exists
            (fun ((x, y, z) : position) ->
              (x = x1 && y = y1 && z = z1) || (x = x2 && y = y2 && z = z2))
            h
        then
          merge pos1 pos2
            (List.sort_uniq
               (fun ((x_1, y_1, z_1) : position) ((x_2, y_2, z_2) : position) ->
                 if x_1 = x_2 && y_1 = y_2 then z_1 - z_2
                 else if x_1 = x_2 then y_1 - y_2
                 else x_1 - x_2)
               (h @ initial))
            t acc
        else merge pos1 pos2 initial t (h :: acc)
  in
  let rec insert_positions ((x1, y1, z1) as pos1 : position)
      ((x2, y2, z2) as pos2 : position) (connections_left : position list list)
      (acc : position list list) : position list list =
    match connections_left with
    | [] -> [ pos1; pos2 ] :: acc
    | h :: t ->
        if
          List.exists
            (fun ((x, y, z) : position) ->
              (x = x1 && y = y1 && z = z1) || (x = x2 && y = y2 && z = z2))
            h
        then
          let added =
            List.sort_uniq
              (fun ((x_1, y_1, z_1) : position) ((x_2, y_2, z_2) : position) ->
                if x_1 = x_2 && y_1 = y_2 then z_1 - z_2
                else if x_1 = x_2 then y_1 - y_2
                else x_1 - x_2)
              (pos1 :: pos2 :: h)
          in
          acc @ merge pos1 pos2 added t []
        else insert_positions pos1 pos2 t (h :: acc)
  in
  let rec build_circuits
      (distances_remaining : (position * position * int) list)
      (acc : position list list)
      (((last_x1, _, _), (last_x2, _, _)) : position * position) : int =
    if List.length acc = 1 then last_x1 * last_x2
    else
      match distances_remaining with
      | [] ->
          raise
            (Failure
               "Ran out of distances before joining all the required boxes")
      | (pos1, pos2, _dist) :: t ->
          build_circuits t (insert_positions pos1 pos2 acc []) (pos1, pos2)
  in
  build_circuits distances
    (positions |> List.map (fun (pos : position) -> [ pos ]))
    ((0, 0, 0), (0, 0, 0))

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
