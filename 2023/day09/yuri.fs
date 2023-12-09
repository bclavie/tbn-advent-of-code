module day09

let input = System.IO.File.ReadAllLines "..//input//day09.txt" |> Seq.map (fun x -> x.Split ' ' |> Array.map int |> List.ofArray) |> List.ofSeq

let rec formHistory (history: int list list) =
    if history.Head |> List.forall (fun x -> x = 0) then history
    else formHistory ((history.Head |> List.pairwise |> List.map (fun (a,b) -> b - a))::history)

let rec predictNextNumber (history: int list list) (lastCalculatedNumber: int) (index: int): int = 
    if index = 0 then ((history[index] |> List.last) + lastCalculatedNumber)
    else predictNextNumber history ((history[index] |> List.last) + lastCalculatedNumber) (index - 1)

let part1 = input |> List.map (fun x -> formHistory [x]) |> List.map (fun x -> predictNextNumber x 0 (x.Length - 1)) |> List.sum

let rec predictPrecedingNumber (history: int list list) (lastCalculatedNumber: int) (index: int): int = 
    if index = (history.Length - 1) then ((history[index] |> List.head) - lastCalculatedNumber)
    else predictPrecedingNumber history ((history[index] |> List.head) - lastCalculatedNumber) (index + 1)

let part2 = input |> List.map (fun x -> formHistory [x]) |> List.map (fun x -> predictPrecedingNumber x 0 0) |> List.sum