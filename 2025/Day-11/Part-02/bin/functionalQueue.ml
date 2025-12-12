type 'a queue = { front : 'a list; back : 'a list }

let push (v : 'a) (q : 'a queue) : 'a queue = { q with back = v :: q.back }

let normalize (q : 'a queue) : 'a queue =
  match (q.front, q.back) with
  | [], _ -> { front = List.rev q.back; back = [] }
  | _ -> q

let pop (q : 'a queue) : 'a option * 'a queue =
  let q = normalize q in
  match q.front with
  | h :: t -> (Option.Some h, { q with front = t })
  | [] -> (Option.None, q)

let empty () : 'a queue = { front = []; back = [] }
