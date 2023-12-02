module Day02

open System.Text.RegularExpressions

let input = System.IO.File.ReadAllLines "input/day02.txt" |> Seq.ofArray

let convertLineToCounts line: (string * int) seq =
    let regex = Regex "\d+ [a-z]*"
    regex.Matches line
        |> Seq.map (fun x -> x.Value.Split(' '))
        |> Seq.map (fun x -> (x[1], int x[0]))

let isGamePossible maxRed maxGreen maxBlue (game: (string * int) seq) : bool = 
    game
    |> Seq.filter (fun (colour,count) -> 
        match (colour,count) with
        | (colour,count) when colour = "red" -> count > maxRed
        | (colour,count) when colour = "green" -> count > maxGreen
        | (colour,count) when colour = "blue" -> count > maxBlue
        | (_,_) -> false)
    |> Seq.length <= 0

let sumOfGameIdsPossible maxRed maxGreen maxBlue (games: (string * int) seq seq) =
    games
    |> Seq.indexed
    |> Seq.filter (fun (_, game) -> isGamePossible maxRed maxGreen maxBlue game)
    |> Seq.sumBy (fun (i, _) -> i+1)

let part1 = 
    input 
    |> Seq.map convertLineToCounts
    |> sumOfGameIdsPossible 12 13 14

printfn "%A" part1

let maxOfColour (game: (string * int) seq) (colour: string): int =
    game
    |> Seq.filter (fun (col, count) -> col = colour)
    |> Seq.map (fun (col, count) -> count) 
    |> Seq.max

let powerForGame (game: (string * int) seq): int =
    ["red"; "green"; "blue"]
    |> Seq.map (fun x -> maxOfColour game x)
    |> Seq.fold (*) 1

let part2 =
    input 
    |> Seq.map convertLineToCounts
    |> Seq.map powerForGame
    |> Seq.sum

printfn "%A" part2