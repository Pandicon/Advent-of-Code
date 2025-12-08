let shift_empty_list_left () =
  let input = [] and expected = [] in
  let result = Aoc_2025__day_04__part_02.Solution.shift_list_left input in
  Alcotest.check
    (Alcotest.list Alcotest.int)
    "Shifting '[]' left results in '[]'" expected result

let shift_nonempty_list_left () =
  let input = [ 1; 2; 3; 4; 5 ] and expected = [ 2; 3; 4; 5; 1 ] in
  let result = Aoc_2025__day_04__part_02.Solution.shift_list_left input in
  Alcotest.check
    (Alcotest.list Alcotest.int)
    "Shifting '[1; 2; 3; 4; 5]' left results in '[2; 3; 4; 5; 1]'" expected
    result

let shift_empty_list_right () =
  let input = [] and expected = [] in
  let result = Aoc_2025__day_04__part_02.Solution.shift_list_right input in
  Alcotest.check
    (Alcotest.list Alcotest.int)
    "Shifting '[]' right results in '[]'" expected result

let shift_nonempty_list_right () =
  let input = [ 1; 2; 3; 4; 5 ] and expected = [ 5; 1; 2; 3; 4 ] in
  let result = Aoc_2025__day_04__part_02.Solution.shift_list_right input in
  Alcotest.check
    (Alcotest.list Alcotest.int)
    "Shifting '[1; 2; 3; 4; 5]' right results in '[5; 1; 2; 3; 4]'" expected
    result

let shift_list_left_two () =
  let input = [ 1; 2; 3; 4; 5 ] and expected = [ 3; 4; 5; 1; 2 ] in
  let result =
    Aoc_2025__day_04__part_02.Solution.shift_list_left_amount 2 input
  in
  Alcotest.check
    (Alcotest.list Alcotest.int)
    "Shifting '[1; 2; 3; 4; 5]' left twice results in '[3; 4; 5; 1; 2]'"
    expected result

let shift_list_left_minus_two () =
  let input = [ 1; 2; 3; 4; 5 ] and expected = [ 4; 5; 1; 2; 3 ] in
  let result =
    Aoc_2025__day_04__part_02.Solution.shift_list_left_amount (-2) input
  in
  Alcotest.check
    (Alcotest.list Alcotest.int)
    "Shifting '[1; 2; 3; 4; 5]' left minus twice results in '[4; 5; 1; 2; 3]'"
    expected result

let test_example_input () =
  let expected = 43 in
  let result =
    Aoc_2025__day_04__part_02.Solution.read_input true
    |> Aoc_2025__day_04__part_02.Solution.solve
  in
  Alcotest.check Alcotest.int "The example input should result in 43" expected
    result

let sum_two_grids () =
  let input = [ [ [ 2; 3; 1 ]; [ 1; 0; -1 ] ]; [ [ 1; 5; -2 ]; [ -2; 1; 1 ] ] ]
  and zero_grid = [ [ 0; 0; 0 ]; [ 0; 0; 0 ] ]
  and expected = [ [ 3; 8; -1 ]; [ -1; 1; 0 ] ] in
  let result =
    Aoc_2025__day_04__part_02.Solution.apply_f_to_elements_of_grids zero_grid
      (fun (a : int) (b : int) -> a + b)
      input
  in
  Alcotest.check
    (Alcotest.list (Alcotest.list Alcotest.int))
    "Summing grids failed" expected result

let multiply_two_grids () =
  let input = [ [ [ 2; 3; 1 ]; [ 1; 0; -1 ] ] ]
  and base_grid = [ [ 0; 1; 1 ]; [ 1; 0; 1 ] ]
  and expected = [ [ 0; 3; 1 ]; [ 1; 0; -1 ] ] in
  let result =
    Aoc_2025__day_04__part_02.Solution.apply_f_to_elements_of_grids base_grid
      (fun (a : int) (b : int) -> a * b)
      input
  in
  Alcotest.check
    (Alcotest.list (Alcotest.list Alcotest.int))
    "Summing grids failed" expected result

let () =
  Alcotest.run "AoC 2025 Day 04 Part 2 tests"
    [
      ( "Shift list left",
        [
          Alcotest.test_case "Shift empty list left" `Quick
            shift_empty_list_left;
          Alcotest.test_case "Shift non-empty list left" `Quick
            shift_nonempty_list_left;
        ] );
      ( "Shift list right",
        [
          Alcotest.test_case "Shift empty list right" `Quick
            shift_empty_list_right;
          Alcotest.test_case "Shift non-empty list right" `Quick
            shift_nonempty_list_right;
        ] );
      ( "Shift list",
        [
          Alcotest.test_case "Shift non-empty list left twice" `Quick
            shift_list_left_two;
          Alcotest.test_case "Shift non-empty list right twice" `Quick
            shift_list_left_minus_two;
        ] );
      ( "Manipulate grids",
        [
          Alcotest.test_case "Sum two grids" `Quick sum_two_grids;
          Alcotest.test_case "Multiply two grids" `Quick multiply_two_grids;
        ] );
      ( "Test example input",
        [ Alcotest.test_case "Test example input" `Quick test_example_input ] );
    ]
