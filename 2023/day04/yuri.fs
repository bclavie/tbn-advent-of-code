module Day4

let input = System.IO.File.ReadAllLines "input\day04.txt"

type ScratchCard = { CardNum: int; WinningNumbers: int Set; MyNumbers: int Set; }

let parseNumbers (str: string) =
    str.Trim().Replace("  ", " ").Split(' ') |> List.ofArray |> List.map (fun x -> x.Trim() |> int) |> Set.ofList

let parseScratchCard (lineIndex: int, line: string) : ScratchCard =
    let split = line.[line.IndexOf(':') + 1..].Split('|')
    { CardNum = lineIndex + 1; WinningNumbers = split[0] |> parseNumbers; MyNumbers = split[1] |> parseNumbers; }

let score (scratchCard: ScratchCard): int =
    pown 2 ((Set.intersect scratchCard.WinningNumbers scratchCard.MyNumbers |> Set.count) - 1)

let scratchCards = input |> List.ofArray |> List.indexed |> List.map  parseScratchCard
let part1 = scratchCards |> List.sumBy score
printfn "Part 1: %A" part1

let countMatchingNumbers scratchCard = 
    Set.intersect scratchCard.WinningNumbers scratchCard.MyNumbers |> Set.count

let countTotalOfAllCards cards =
    let matchesPerCard = cards |> List.map countMatchingNumbers
    let amountPerCard = List.init (cards |> List.length) (fun _ -> 1)

    let fld (apc: int list) (idx: int, amount: int) : int list =
        apc 
        |> List.indexed
        |> List.map (fun (index, value) ->
            if index > idx && index <= idx + amount // in the range of indices to be adjusted  
            then value + (1 * apc[idx]) // increase by amount of cards in current index
            else value)

    matchesPerCard
    |> List.indexed
    |> List.fold fld amountPerCard
    |> List.sum

let part2 = countTotalOfAllCards scratchCards 
printfn "Part 2: %A" part2