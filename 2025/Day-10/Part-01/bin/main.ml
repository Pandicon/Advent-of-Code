let run_example = false

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let parse_line (line : string) : int list * int list list * int list =
  match line |> String.split_on_char ' ' with
  | [] -> raise (Failure ("Line '" ^ line ^ "' is of invalid format"))
  | h :: t -> (
      let on_off =
        h |> String.to_seq |> List.of_seq
        |> List.filter (fun (c : char) -> c <> '[' && c <> ']')
        |> List.map (fun (c : char) ->
               match c with
               | '.' -> 0
               | '#' -> 1
               | _ ->
                   raise
                     (Failure ("Invalid character '" ^ String.make 1 c ^ "'")))
      in
      match List.rev t with
      | [] -> raise (Failure ("Line '" ^ line ^ "' is of invalid format"))
      | h :: t ->
          let joltages =
            h |> String.to_seq
            |> Seq.filter (fun (c : char) -> c <> '{' && c <> '}')
            |> String.of_seq |> String.split_on_char ','
            |> List.map int_of_string
          in
          let toggles =
            t
            |> List.map (fun (s : string) ->
                   s |> String.to_seq
                   |> Seq.filter (fun (c : char) -> c <> '(' && c <> ')')
                   |> String.of_seq |> String.split_on_char ','
                   |> List.map int_of_string)
          in
          (on_off, toggles, joltages))

let rec generate_subsets (l : 'a list) : 'a list list =
  match l with
  | [] -> [ [] ]
  | h :: t ->
      let without_h = generate_subsets t in
      without_h @ (without_h |> List.map (fun (s : 'a list) -> h :: s))

let rec apply_toggles (toggles : int list) (state_rem : int list)
    (state_acc : int list) (i : int) : int list =
  match (toggles, state_rem) with
  | [], _ -> List.rev state_acc @ state_rem
  | h :: t, v :: ts ->
      if h = i then apply_toggles t ts ((1 - v) :: state_acc) (i + 1)
      else apply_toggles toggles ts (v :: state_acc) (i + 1)
  | _ -> raise (Failure "Invalid state")

let rec evaluate_subset (toggles : int list list) (state : int list) : int list
    =
  match toggles with
  | [] -> state
  | h :: t ->
      let new_state = apply_toggles h state [] 0 in
      evaluate_subset t new_state

let solve_line
    ((on_off, toggles, _joltages) : int list * int list list * int list) : int =
  let subsets = generate_subsets toggles
  and default = on_off |> List.map (fun _a -> 0) in
  subsets
  |> List.map (fun (subset : int list list) ->
         (List.length subset, evaluate_subset subset default))
  |> List.filter (fun ((_l, res) : int * int list) -> res = on_off)
  |> List.fold_left
       (fun (acc : int) ((l, _res) : int * int list) -> min acc l)
       (List.length toggles)

let solve (input : string) : int =
  input |> String.split_on_char '\n' |> List.map String.trim
  |> List.map parse_line |> List.map solve_line
  |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
