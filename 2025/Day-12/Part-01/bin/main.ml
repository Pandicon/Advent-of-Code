let run_example = false

type res = { yes : int; no : int; maybe : int }
type task = { width : int; height : int; counts : int list }
type box = { width : int; height : int; coverage : int list list }

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let parse_tasks (tasks_lines : string) : task list =
  let parse_task (task_line : string) : task =
    let (width, height), counts =
      match task_line |> Str.split (Str.regexp_string ": ") with
      | [ s; c ] ->
          let w, h =
            match s |> String.split_on_char 'x' with
            | [ w; h ] -> (w, h)
            | _ ->
                raise (Failure ("Task '" ^ task_line ^ "' has invalid format"))
          in
          let counts =
            c |> String.split_on_char ' '
            |> List.filter (( <> ) "")
            |> List.map int_of_string
          in
          ((int_of_string w, int_of_string h), counts)
      | _ -> raise (Failure ("Task '" ^ task_line ^ "' has invalid format"))
    in
    { width; height; counts }
  in
  tasks_lines |> String.split_on_char '\n' |> List.map String.trim
  |> List.map parse_task

let rotate_90_deg (l : 'a list list) (h : int) : 'a list list =
  l
  |> List.fold_left
       (fun (acc : 'a list list) (row : 'a list) ->
         List.map2 (fun (new_row : 'a list) (v : 'a) -> v :: new_row) acc row)
       (List.init h (fun _ -> []))
  |> List.map List.rev

let parse_boxes (boxes_strings : string list) : box list =
  let parse_box (box_string : string) : box =
    match box_string |> String.trim |> String.split_on_char '\n' with
    | _h :: t ->
        let coverage =
          t |> List.map String.trim
          |> List.map (fun (l : string) ->
                 l |> String.to_seq |> List.of_seq
                 |> List.map (fun (c : char) ->
                        match c with
                        | '.' -> 0
                        | '#' -> 1
                        | _ ->
                            raise
                              (Failure
                                 ("Unexpected character '" ^ String.make 1 c
                                ^ "'"))))
        in
        let w, h =
          match coverage with
          | [] -> (0, 0)
          | h :: _ -> (List.length h, List.length t)
        in
        let w, h, coverage =
          if w >= h then (w, h, coverage) else (h, w, rotate_90_deg coverage h)
        in
        { width = w; height = h; coverage }
    | _ -> raise (Failure ("Box '" ^ box_string ^ "' has invalid format"))
  in
  boxes_strings |> List.map parse_box

let solve_task (boxes : box list) (t : task) : res =
  let total_boxes =
    t.counts |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0
  in
  let total_squares =
    List.map2
      (fun (b : box) (count : int) ->
        let squares =
          b.coverage |> List.flatten
          |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0
        in
        squares * count)
      boxes t.counts
    |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0
  in
  let max_w, max_h =
    boxes
    |> List.fold_left
         (fun ((curr_max_w, curr_max_h) : int * int) (b : box) ->
           (max curr_max_w b.width, max curr_max_h b.height))
         (0, 0)
  in
  if
    t.width / max_w * (t.height / max_h) >= total_boxes
    || t.width / max_h * (t.height / max_w) >= total_boxes
  then { yes = 1; no = 0; maybe = 0 }
  else if total_squares > t.width * t.height then { yes = 0; no = 1; maybe = 0 }
  else { yes = 0; no = 0; maybe = 1 }

let solve (input : string) : res =
  let boxes, tasks =
    match input |> Str.split (Str.regexp_string "\n\n") |> List.rev with
    | h :: t ->
        let tasks = h |> parse_tasks in
        let boxes = t |> List.rev |> parse_boxes in
        (boxes, tasks)
    | _ -> raise (Failure "Invalid input format")
  in
  tasks
  |> List.map (solve_task boxes)
  |> List.fold_left
       (fun (acc : res) (r : res) ->
         {
           yes = acc.yes + r.yes;
           no = acc.no + r.no;
           maybe = acc.maybe + r.maybe;
         })
       { yes = 0; no = 0; maybe = 0 }

let main () : res = read_input run_example |> solve

let () =
  let r = main () in
  print_endline
    ("Yes: " ^ string_of_int r.yes ^ ", No: " ^ string_of_int r.no ^ ", Maybe: "
   ^ string_of_int r.maybe)
