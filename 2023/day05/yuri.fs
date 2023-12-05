module Day05

let input =
    System.IO.File.ReadAllLines "..//input//example05.txt" 
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

type State = { LowestSeed: uint; CurrentSeed: uint; MaxSeed: uint; }
let initialStates = seeds |> List.chunkBySize 2 |> List.map (fun x -> { LowestSeed = x[0]; CurrentSeed = x[0]; MaxSeed = x[0] + x[1] })

let rec lowestFinalSeedForPair (state: State): State = 
    if state.CurrentSeed = state.MaxSeed then state
    else 
        lowestFinalSeedForPair {
            LowestSeed = min state.LowestSeed (applyAllCategories state.CurrentSeed mapsByCategory);
            CurrentSeed = state.CurrentSeed + (uint 1);
            MaxSeed = state.MaxSeed;
        }

let part2 = initialStates |> List.map lowestFinalSeedForPair |> List.map (fun x -> x.LowestSeed) |> List.min
printf "part 2: %A" part2