let input = System.IO.File.ReadAllLines("input/day01.txt")

let digits = "123456789".ToCharArray()

let part1 = 
    input
    |> Array.map (fun x -> int (x.[x.IndexOfAny(digits)].ToString() + x.[x.LastIndexOfAny(digits)].ToString()))
    |> Array.sum

printfn "Part 1: %A" part1

let part2Digits = ["1"; "2"; "3"; "4"; "5"; "6"; "7"; "8"; "9"; "one"; "two"; "three"; "four"; "five"; "six"; "seven"; "eight"; "nine"]

let digitToString str =
    match str with 
    | "1" | "one" -> "1"
    | "2" | "two" -> "2"
    | "3" | "three" -> "3"
    | "4" | "four" -> "4"
    | "5" | "five" -> "5"
    | "6" | "six" -> "6"
    | "7" | "seven" -> "7"
    | "8" | "eight" -> "8"
    | "9" | "nine" -> "9"
    | _ -> failwith "Invalid spelled out digit"

type DigitIndices = { Value: string; FirstIndex: int; LastIndex: int }

let firstNumber indices =
    (indices |> List.sortBy (fun x -> x.FirstIndex) |> List.head).Value

let lastNumber indices =
    (indices |> List.sortByDescending (fun x -> x.LastIndex) |> List.head).Value

let lineToNumber (line: string) : int = 
        part2Digits 
        |> List.map (fun str -> { Value = digitToString str; FirstIndex = line.IndexOf(str); LastIndex = line.LastIndexOf(str) })
        |> List.filter (fun x -> x.FirstIndex > -1 && x.LastIndex > -1)
        |> (fun x -> [firstNumber x ; lastNumber x ] |> String.concat "") |> int

let part2 =
    input 
    |> Array.map lineToNumber
    |> Array.sum

printfn "Part 2: %A" part2