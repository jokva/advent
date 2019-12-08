open System.Collections.Generic

let rec readlines () = seq {
    let line = System.Console.ReadLine()
    if line <> null then
        yield line
        yield! readlines ()
}

let rec traverse(orbits: IDictionary<'a, 'a>, body: 'a) =
    if not (orbits.ContainsKey(body))
    then 1
    else 1 + traverse(orbits, orbits.[body])

let rec path(orbits: IDictionary<'a, 'a>, dst: 'a, src: 'a) =
    if src = dst then [src]
    else src :: path(orbits, dst, orbits.[src])

[<EntryPoint>]
let main argv =
    let orbits = readlines()
               |> Seq.map (fun x -> x.Split(')'))
               |> Seq.map (fun x -> x.[1], x.[0])
               |> dict

    let count = orbits
              |> Seq.map (fun (KeyValue(_,v)) -> traverse(orbits, v))
              |> Seq.sum

    printfn "Number of orbits: %d" count

    let you = List.rev(path(orbits, "COM", "YOU"))
    let san = List.rev(path(orbits, "COM", "SAN"))

    let shared_path_steps = Seq.zip you san
                          |> Seq.takeWhile (fun (x, y) -> x = y)
                          |> Seq.length
    let xfers = (you.Length + san.Length) - 2 * (1 + shared_path_steps)
    printfn "Minimum transfers requried: %d" xfers
    0
