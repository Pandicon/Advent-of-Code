let run_example = false

let main () : int =
  Aoc_2025__day_04__part_02.Solution.read_input run_example
  |> Aoc_2025__day_04__part_02.Solution.solve

let () = main () |> string_of_int |> print_endline
