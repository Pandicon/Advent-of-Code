type 'a fArray = Empty | Data of ('a * 'a fArray * 'a fArray)

let rec get (i : int) (arr : 'a fArray) : 'a option =
  match arr with
  | Empty -> None
  | Data (v, left, right) ->
      if i = 0 then Some v
      else if i mod 2 = 1 then get ((i - 1) / 2) left
      else get ((i - 1) / 2) right

exception InvalidInsertIndex

let insert (arr : 'a fArray) (i : int) (value : 'a) : ('a fArray, unit) result =
  let rec inner (arr : 'a fArray) (i : int) : 'a fArray =
    match arr with
    | Empty ->
        if i = 0 then Data (value, Empty, Empty) else raise InvalidInsertIndex
    | Data (v, left, right) ->
        if i = 0 then Data (value, left, right)
        else if i mod 2 = 1 then Data (v, inner left ((i - 1) / 2), right)
        else Data (v, left, inner right ((i - 1) / 2))
  in
  try Result.ok (inner arr i) with InvalidInsertIndex -> Result.error ()

let from_list (l : 'a list) : 'a fArray =
  let rec inner (l_rem : 'a list) (i : int) (acc : 'a fArray) =
    match l_rem with
    | [] -> acc
    | h :: t ->
        inner t (i + 1)
          (match insert acc i h with
          | Result.Ok v -> v
          | Result.Error () ->
              raise (Failure "Failed to convert the list to an array"))
  in
  inner l 0 Empty
