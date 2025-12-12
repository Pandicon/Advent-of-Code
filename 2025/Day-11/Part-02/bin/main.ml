module StringMap = Map.Make (String)

let run_example = false

type 'a node = { name : 'a; paths : 'a list }

let read_file (filename : string) : string =
  In_channel.with_open_bin filename In_channel.input_all

let remove_character_occurences (c : char) (s : string) : string =
  s |> String.to_seq |> Seq.filter (fun ch -> ch <> c) |> String.of_seq

let read_input (run_example : bool) : string =
  (if run_example then "example-input.txt" else "input.txt")
  |> read_file
  |> remove_character_occurences '\r'

let parse_line (line : string) : string node option =
  match line |> String.split_on_char ':' with
  | [ n; rest ] -> (
      match
        rest |> String.trim |> String.split_on_char ' '
        |> List.filter (fun (s : string) -> s <> "")
      with
      | [] -> Option.None
      | p -> Option.Some { name = n; paths = p })
  | _ -> raise (Failure ("Line '" ^ line ^ "' is of invalid format"))

let find_paths (start : int) (finish : int) (adj : int list array)
    (ord : int list) : int =
  let paths = Array.init (Array.length adj) (fun (_ : int) -> 0) in
  paths.(start) <- 1;
  let _ =
    ord
    |> List.map (fun (node_id : int) ->
           let _new_paths =
             adj.(node_id)
             |> List.fold_left
                  (fun (curr_paths : int array) (adj_id : int) ->
                    curr_paths.(adj_id) <-
                      curr_paths.(adj_id) + curr_paths.(node_id);
                    curr_paths)
                  paths
           in
           ())
  in
  paths.(finish)

let topological_order (adj : int list array) : int list =
  let rec inner (q : int FunctionalQueue.queue)
      (nodes_weights : (int * int option) array) (ord : int list) : int list =
    match FunctionalQueue.pop q with
    | Option.Some node_i, q ->
        let new_weights =
          nodes_weights
          |> Array.map (fun ((i, w) : int * int option) ->
                 if List.mem i adj.(node_i) then
                   (i, Option.map (fun (a : int) -> a - 1) w)
                 else (i, w))
        in
        let new_q =
          new_weights
          |> Array.fold_left
               (fun (q_acc : int FunctionalQueue.queue)
                    ((i, w) : int * int option) ->
                 match w with
                 | Option.Some 0 -> FunctionalQueue.push i q_acc
                 | _ -> q_acc)
               q
        in
        let new_weights =
          new_weights
          |> Array.map (fun ((i, w) : int * int option) ->
                 match w with Option.Some 0 -> (i, None) | _ -> (i, w))
        in
        inner new_q new_weights (node_i :: ord)
    | Option.None, _ -> List.rev ord
  in
  let nodes_weights =
    adj
    |> Array.fold_left
         (fun (weights : (int * int option) array) (adjancent : int list) ->
           let _ =
             adjancent
             |> List.map (fun (i : int) ->
                    let _, old_w = weights.(i) in
                    weights.(i) <- (i, Option.map (fun (v : int) -> v + 1) old_w))
           in
           weights)
         (Array.init (Array.length adj) (fun _ -> Option.Some 0)
         |> Array.mapi (fun (i : int) (w : int option) -> (i, w)))
  in
  let q = FunctionalQueue.empty () in
  let new_q =
    nodes_weights
    |> Array.fold_left
         (fun (q_acc : int FunctionalQueue.queue) ((i, w) : int * int option) ->
           match w with
           | Option.Some 0 -> FunctionalQueue.push i q_acc
           | _ -> q_acc)
         q
  in
  let new_weights =
    nodes_weights
    |> Array.map (fun ((i, w) : int * int option) ->
           match w with Option.Some 0 -> (i, None) | _ -> (i, w))
  in
  inner new_q new_weights []

let solve (input : string) : int =
  let nodes =
    input |> String.split_on_char '\n' |> List.filter_map parse_line
  in
  let nodes_all =
    nodes
    |> List.map (fun (n : string node) -> n.name :: n.paths)
    |> List.flatten
    |> List.sort_uniq (fun (n1 : string) (n2 : string) -> compare n1 n2)
  in
  let str_name_to_int =
    nodes_all
    |> List.mapi (fun (i : int) (n : string) -> (i, n))
    |> List.fold_left
         (fun (acc : int StringMap.t) ((i, n) : int * string) ->
           StringMap.add n i acc)
         StringMap.empty
  in
  let svr_i = StringMap.find "svr" str_name_to_int
  and fft_i = StringMap.find "fft" str_name_to_int
  and dac_i = StringMap.find "dac" str_name_to_int
  and out_i = StringMap.find "out" str_name_to_int in
  let nodes =
    nodes
    |> List.map (fun (n : string node) ->
           {
             name = StringMap.find n.name str_name_to_int;
             paths =
               n.paths
               |> List.map (fun (name : string) ->
                      StringMap.find name str_name_to_int);
           })
  in
  let adjancencies =
    nodes
    |> List.fold_left
         (fun (acc : int list array) (n : int node) ->
           acc.(n.name) <- n.paths;
           acc)
         (Array.init (nodes_all |> List.length) (fun _i -> []))
  in
  let top_ord = topological_order adjancencies in
  let fft_to_dac = find_paths fft_i dac_i adjancencies top_ord in
  if fft_to_dac = 0 then
    let svr_to_dac = find_paths svr_i dac_i adjancencies top_ord
    and dac_to_fft = find_paths dac_i fft_i adjancencies top_ord
    and fft_to_out = find_paths fft_i out_i adjancencies top_ord in
    svr_to_dac * dac_to_fft * fft_to_out
  else
    let svr_to_fft = find_paths svr_i fft_i adjancencies top_ord
    and dac_to_out = find_paths dac_i out_i adjancencies top_ord in
    svr_to_fft * fft_to_dac * dac_to_out

let main () : int = read_input run_example |> solve
let () = main () |> string_of_int |> print_endline
