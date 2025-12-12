let run_example = false

type node = { name : string; paths : string list }

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let parse_line (line : string) : node option =
  match line |> String.split_on_char ':' with
  | [ n; rest ] -> (
      match
        rest |> String.trim |> String.split_on_char ' '
        |> List.filter (fun (s : string) -> s <> "")
      with
      | [] -> Option.None
      | p -> Option.Some { name = n; paths = p })
  | _ -> raise (Failure ("Line '" ^ line ^ "' is of invalid format"))

let rec find_paths (current_node : string) (current_path : string list)
    (paths_num : int) (nodes : node list) : int =
  if current_node = "out" then paths_num + 1
  else if List.mem current_node current_path then
    raise (Failure "Cycle encountered")
    (* Should not happen since then the number of paths would be infinite *)
  else
    match List.find_opt (fun (n : node) -> n.name = current_node) nodes with
    | Option.None ->
        raise (Failure ("Node '" ^ current_node ^ "' does not exist"))
    | Option.Some node ->
        let ways_out = node.paths in
        ways_out
        |> List.fold_left
             (fun (current_paths_count : int) (way_out : string) ->
               find_paths way_out
                 (current_node :: current_path)
                 current_paths_count nodes)
             paths_num

let solve (input : string) : int =
  let nodes =
    input |> String.split_on_char '\n' |> List.filter_map parse_line
  in
  nodes |> find_paths "you" [] 0

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
