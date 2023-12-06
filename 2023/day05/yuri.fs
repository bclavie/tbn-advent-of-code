module Day05

let input =
    System.IO.File.ReadAllLines "..//input//day05.txt"
    |> List.ofSeq

type MapRange = { DestinationStart: uint; SourceStart: uint; Length: uint; }

let parseToMapRange (str: string list) =
    str
    |> List.map (fun x -> x.Split(" ") |> Array.map uint)
    |> List.map (fun x -> { DestinationStart = x[0]; SourceStart = x[1]; Length = x[2] })

let seeds = input[0] |> (fun str -> str.Substring(7).Trim().Split(" ") |> List.ofSeq |> List.map uint)

let inputWithoutSeeds = input |> List.skip(2)
let categoryIndices : uint list = inputWithoutSeeds |> List.indexed |> List.filter (fun (_, str) -> str.IndexOf(":") >= 0) |> List.map (fun (idx, _) -> uint idx)
let mapsByCategory : MapRange list list =
    categoryIndices |> List.map (fun idx -> inputWithoutSeeds |> List.skip(int idx + 1) |> List.takeWhile (fun x -> x <> "") |> parseToMapRange)

let applyCategory (seed: uint) (categoryMaps: MapRange list) : uint = 
    let adjustSeed (seed: uint) (map: MapRange) : uint =
        seed + map.DestinationStart - map.SourceStart
        
    let fittingMaps = categoryMaps |> List.filter (fun x -> x.SourceStart <= seed && x.SourceStart + x.Length > seed)
    if fittingMaps |> List.isEmpty then seed
    else fittingMaps |> List.head |> adjustSeed seed

// Run each seed through every set of maps and get the final result for each
let applyAllCategories(seed: uint) (allCategories: MapRange list list) =
    allCategories |> List.fold (fun seed listOfMaps -> applyCategory seed listOfMaps) seed

let part1 = seeds |> List.map (fun seed -> applyAllCategories seed mapsByCategory) |> List.min
printf "part 1: %A" part1

type State = { RangeStart: uint; Length: uint; }
let initialStates = seeds |> List.chunkBySize 2 |> List.map (fun x -> { RangeStart = x[0]; Length = x[1] })

let rec stepSizeApproximation (states: State list) (stepSize: uint) (bestEstimate: uint) =
    let endOfRange x = x.Length + x.RangeStart
    let startOfRange x = x.RangeStart
    let rangeMax = if bestEstimate = (uint -1) then states |> List.maxBy endOfRange |> endOfRange else bestEstimate + (stepSize * uint 10)
    let rangeMin = if bestEstimate = (uint -1) then states |> List.minBy startOfRange |> startOfRange else bestEstimate - (stepSize * uint 10)
    
    List.init (float (rangeMax - rangeMin) / (float stepSize / float 10) |> ceil |> int) (fun idx -> rangeMin + (stepSize * (uint idx))) 
        |> List.filter (fun x -> states |> List.exists (fun state -> x > state.RangeStart && x < state.RangeStart + state.Length))
        |> List.map (fun seed -> (seed, applyAllCategories seed mapsByCategory)) |> List.minBy (fun (seed, outcome) -> outcome)
        |> (fun (seed, lowestOutcome) -> if stepSize < (uint 10) then lowestOutcome else stepSizeApproximation states (stepSize / uint 10) seed)

let initialStepSize = (initialStates |> (fun x -> (x |> List.sumBy (fun x -> x.Length)) / (uint initialStates.Length)))
let approx = (stepSizeApproximation initialStates (initialStepSize) (-1 |> uint)) |> int
printf "part 2: %A" approx
