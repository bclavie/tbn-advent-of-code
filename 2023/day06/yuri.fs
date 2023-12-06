module yuri

open System.Text.RegularExpressions

type RaceRecord = { Time: uint64; Distance: uint64; }

let input = System.IO.File.ReadAllLines "..//input//day06.txt" |> List.ofSeq

let digitRegex = new Regex @"[ ]\d+"
let parseInput (inputLines: string list): RaceRecord list = 
    let times = (digitRegex.Matches inputLines[0]) |> Seq.map (fun x -> int x.Value) |> List.ofSeq
    let distances = (digitRegex.Matches inputLines[1]) |> Seq.map (fun x -> int x.Value) |> List.ofSeq
    times |> List.indexed |> List.map (fun (idx, time) -> { Time = uint64 time; Distance = uint64 distances[idx] })

let records = parseInput input

let totalDistance raceTime holdTime = 
    (raceTime - holdTime) * holdTime

let calculateWaysToWin (raceRecord: RaceRecord): int =
    List.init (int raceRecord.Time - 1) (fun idx -> idx+1)
    |> List.map (fun holdTime -> totalDistance raceRecord.Time (uint64 holdTime))
    |> List.filter (fun totalDistance -> totalDistance > raceRecord.Distance)
    |> List.length

let part1 = 
    records 
    |> List.map calculateWaysToWin
    |> List.fold (*) 1

let part2RaceRecord =
    let time = digitRegex.Matches input[0] |> Seq.map (fun m -> m.Value.Trim()) |> String.concat "" |> uint64
    let distance = digitRegex.Matches input[1] |> Seq.map (fun m -> m.Value.Trim()) |> String.concat "" |> uint64
    { Time = time; Distance = distance }

let lowestHoldTimeToWin raceTime distance =
    (raceTime - sqrt (raceTime * raceTime - 4.0 * (distance + 1.0)))

let part2BruteForce = calculateWaysToWin part2RaceRecord 
let part2QuickMaffs = (part2RaceRecord.Time - (ceil (lowestHoldTimeToWin (float part2RaceRecord.Time) (float part2RaceRecord.Distance)) |> uint64)) |> int
