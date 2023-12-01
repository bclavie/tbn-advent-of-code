let input = System.IO.File.ReadAllLines("day01/input.txt")

let digits = "123456789".ToCharArray()
let spelledOutDigits = ["one"; "two"; "three"; "four"; "five"; "six"; "seven"; "eight"; "nine"]

let part1 = 
    input
    |> Array.map (fun x -> int (x.[x.IndexOfAny(digits)].ToString() + x.[x.LastIndexOfAny(digits)].ToString()))
    |> Array.sum

printfn "Part 1: %A" part1

let spelledOutDigitValue str =
    match str with 
    | "one" -> 1
    | "two" -> 2
    | "three" -> 3
    | "four" -> 4
    | "five" -> 5
    | "six" -> 6
    | "seven" -> 7
    | "eight" -> 8
    | "nine" -> 9
    | _ -> failwith "Invalid spelled out digit"

type DigitIndices = { Value: int; FirstIndex: int; LastIndex: int }

let lineToNumber (line: string) = 
    let spelledOut = 
        spelledOutDigits 
        |> List.map (fun str -> { Value = spelledOutDigitValue str; FirstIndex = line.IndexOf(str); LastIndex = line.LastIndexOf(str) })

    let numbers =
        digits
        |> List.ofArray
        |> List.map (fun c -> { Value = c.ToString() |> int; FirstIndex = line.IndexOf(c); LastIndex = line.LastIndexOf(c) })

    let combined = 
        List.append numbers spelledOut
        |> List.filter (fun x -> x.FirstIndex > -1 && x.LastIndex > -1)

    let firstNumber =
        combined
        |> List.sortBy (fun x -> x.FirstIndex)
        |> List.head
        |> (fun x -> x.Value)

    let lastNumber = 
        combined
        |> List.sortByDescending (fun x -> x.LastIndex)
        |> List.head
        |> (fun x -> x.Value)

    int (firstNumber.ToString() + lastNumber.ToString())

let part2 =
    input 
    |> Array.map lineToNumber
    |> Array.sum

printfn "Part 2: %A" part2