module Day08

type Steps = char list
type Node = { Name: string; Left: string; Right: string; }

let lines = System.IO.File.ReadLines "..//input//day08.txt" |> List.ofSeq

let steps = lines[0].ToCharArray() |> List.ofArray |> List.map char
let nodes = lines |> List.skip(2) |> List.map (fun x -> { Name = x.Substring(0,3); Left = x.Substring(7,3); Right = x.Substring(12,3); })

let start = nodes |> List.skipWhile (fun x -> x.Name <> "AAA") |> List.head

let rec stepsToReachEndPart1 (currentNode: Node) (stepsTaken: int) : int =
    if currentNode.Name = "ZZZ" then stepsTaken 
    else 
        let step = steps[stepsTaken % steps.Length] 
        let nextNode = nodes |> List.find (fun x -> x.Name = (if step = 'L' then currentNode.Left else currentNode.Right))            
        stepsToReachEndPart1 nextNode (stepsTaken + 1)

let rec stepsToReachEndPart2 (currentNode: Node) (stepsTaken: int) : int =
    if currentNode.Name[2] = 'Z' then stepsTaken
    else
        let step = steps[stepsTaken % steps.Length] 
        let nextNode = nodes |> List.find (fun x -> x.Name = (if step = 'L' then currentNode.Left else currentNode.Right))            
        stepsToReachEndPart2 nextNode (stepsTaken + 1)

let rec gcd (a:int64) (b:int64) =   
    printfn "GCD: %A %A" a b
    if b = 0 then a 
    else gcd b (a % b)

let lcm a b =
    (a * b) / (gcd a b)

let distancesToFirstFinish = nodes |> List.filter (fun x -> x.Name[2] = 'A') |> List.map (fun node -> int64 (stepsToReachEndPart2 node 0))

printfn "Part 1: %A" (stepsToReachEndPart1 start 0)
printfn "Part 2: %s" (distancesToFirstFinish |> List.reduce lcm |> string)