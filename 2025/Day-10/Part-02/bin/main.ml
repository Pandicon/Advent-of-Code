let run_example = false

type equation = { vars : int list; rhs : int }

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

let solve_equation_set (eqs : equation list) : int =
  let rec inner (eqs : equation list) (current_best : int) (current_score : int)
      : int =
    if current_score >= current_best then current_best
    else if eqs |> List.exists (fun (eq : equation) -> eq.rhs < 0) then
      current_best (* Can not satisfy the equation if the rhs is negative *)
    else if
      eqs
      |> List.exists (fun (eq : equation) ->
             eq.rhs > 0 && eq.vars |> List.length = 0)
    then current_best
      (* Can not satisfy the equation if the rhs is not zero but I have no variables left *)
    else
      let eqs =
        eqs
        |> List.filter (fun (eq : equation) -> eq.vars |> List.length > 0)
        |> List.sort (fun (eq1 : equation) (eq2 : equation) ->
               (eq1.vars |> List.length) - (eq2.vars |> List.length))
      in
      match eqs with
      (* Solved all equations successfully *)
      | [] -> min current_best current_score
      | curr_eq :: remaining_eqs -> (
          match curr_eq.vars with
          | [] -> inner remaining_eqs current_best current_score
          | current_var_id :: _ ->
              let new_eqs =
                eqs
                |> List.map (fun (eq : equation) ->
                       {
                         eq with
                         vars =
                           eq.vars
                           |> List.filter (fun (i : int) -> i <> current_var_id);
                       })
              in
              let rec try_values (curr_val : int) (previous_best : int) : int =
                let curr_best =
                  inner
                    (List.map2
                       (fun (old_eq : equation) (eq : equation) ->
                         if List.mem current_var_id old_eq.vars then
                           { eq with rhs = eq.rhs - curr_val }
                         else eq)
                       eqs new_eqs)
                    current_best (current_score + curr_val)
                in
                if curr_val >= curr_eq.rhs then min curr_best previous_best
                else try_values (curr_val + 1) (min previous_best curr_best)
              in
              try_values 0 current_best)
  in
  inner eqs
    (eqs |> List.fold_left (fun (acc : int) (eq : equation) -> acc + eq.rhs) 0)
    0

let solve_line
    ((_on_off, toggles, joltages) : int list * int list list * int list) : int =
  let equations =
    joltages
    |> List.mapi (fun (joltage_i : int) (joltage : int) ->
           let vars =
             toggles
             |> List.mapi (fun (var_i : int) (toggle : int list) ->
                    (var_i, List.mem joltage_i toggle))
             |> List.filter_map (fun ((var_i, included) : int * bool) ->
                    if included then Option.Some var_i else Option.None)
           in
           { vars; rhs = joltage })
  in
  equations |> solve_equation_set

let solve (input : string) : int =
  input |> String.split_on_char '\n' |> List.map String.trim
  |> List.map parse_line |> List.map solve_line
  |> List.fold_left (fun (acc : int) (v : int) -> acc + v) 0

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
