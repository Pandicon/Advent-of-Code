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

(* Check for the potential annoying edge case where the whole/a part of the floor could be covered by this:
#XXXXX#
#X##XX#
#X##XX#
#XXXXX#
In which case my algorithm breaks.
However, the input does not include anything like this so my algorithm works (at least for my input).
A more probable edge case building on the same principle would have been this:
##.#X#
X#X#X##
XXXXXXX
#XX#XXX
.#X#XXX
.XXXXXX
.#XXXX#
Where choosing
##.#X#
XOX#X#O
XXXXXXX
#XX#XXX
.#X#XXX
.XXXXXX
.OXXXXO
would be valid but rejected by my algorithm.
*)
let check ((x1, y1) : position) ((x2, y2) : position) : unit =
  if (x1 = x2 && abs (y1 - y2) = 1) || (y1 = y2 && abs (x1 - x2) = 1) then
    print_endline
      ("Annoying: (" ^ string_of_int x1 ^ ", " ^ string_of_int y1 ^ "), ("
     ^ string_of_int x2 ^ ", " ^ string_of_int y2 ^ ")")
  else if x1 <> x2 && y1 <> y2 then
    print_endline
      ("Broken: (" ^ string_of_int x1 ^ ", " ^ string_of_int y1 ^ "), ("
     ^ string_of_int x2 ^ ", " ^ string_of_int y2 ^ ")")

let solve (input : string) : int =
  let points =
    input |> String.split_on_char '\n' |> List.map String.trim
    |> List.map parse_line
  in
  let pairs =
    match points with
    | ph :: pt ->
        List.map2
          (fun (p1 : position) (p2 : position) -> (p1, p2))
          points (pt @ [ ph ])
    | _ -> raise (Failure "No pairs")
  in
  let _ =
    List.map (fun ((p1, p2) : position * position) -> check p1 p2) pairs
  in
  let areas =
    points
    |> List.map (fun ((x1, y1) as point : position) ->
           points
           |> List.map (fun ((x2, y2) as point2 : position) ->
                  let max_x = max x1 x2
                  and min_x = min x1 x2
                  and max_y = max y1 y2
                  and min_y = min y1 y2 in
                  let red_tile_within =
                    List.exists
                      (fun (((xp1, yp1), (xp2, yp2)) : position * position) ->
                        (not ((x1 = xp1 && y1 = yp1) || (x1 = xp2 && y1 = yp2)))
                        && (not
                              ((x2 = xp1 && y2 = yp1) || (x2 = xp2 && y2 = yp2)))
                        && not
                             ((xp1 >= max_x && xp2 >= max_x)
                             || (xp1 <= min_x && xp2 <= min_x)
                             || (yp1 >= max_y && yp2 >= max_y)
                             || (yp1 <= min_y && yp2 <= min_y)))
                      pairs
                  in
                  if red_tile_within then 0
                  else
                    let v = calculate_area point point2 in
                    v))
    |> List.flatten
    |> List.sort (fun (area1 : int) (area2 : int) -> area2 - area1)
  in
  match areas with h :: _ -> h | [] -> raise (Failure "No areas found")

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
