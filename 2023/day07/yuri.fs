module Day07

let input = System.IO.File.ReadAllLines "..//input//day07.txt" |> List.ofSeq 

type Hand = { Cards: char list; Str: string; Bid: int; }

let cardValue (jForJoker: bool) (card: char): int =
    match card with 
    | card when card >= '2' && card <= '9' -> card.ToString() |> int
    | 'T' -> 10
    | 'J' -> if jForJoker then 1 else 11
    | 'Q' -> 12
    | 'K' -> 13
    | 'A' -> 14
    | _ -> failwith $"Unknown card: {card}" 

type HandValue = 
    | FiveOfAKind = 7 
    | FourOfAKind = 6 
    | FullHouse = 5 
    | ThreeOfAKind = 4 
    | TwoPair = 3 
    | OnePair = 2 
    | HighCard = 1

let parseHandType (cardsByOccurrences: (char * int) list) : HandValue =
    match cardsByOccurrences.Head |> snd with 
    | 5 -> HandValue.FiveOfAKind
    | 4 -> HandValue.FourOfAKind
    | 3 when cardsByOccurrences[3] |> snd = 2 -> HandValue.FullHouse
    | 3 -> HandValue.ThreeOfAKind
    | 2 when cardsByOccurrences[2] |> snd = 2 -> HandValue.TwoPair
    | 2 -> HandValue.OnePair
    | 1 -> HandValue.HighCard
    | _ -> failwith $"Failed parsing hand value {cardsByOccurrences}"

let sortByOccurrences hand = 
    hand |> List.map (fun x -> (x, hand |> List.filter (fun y -> x = y) |> List.length)) |> List.sortByDescending(fun (card, occ) -> occ)

let tieBreaker (jForJoker: bool) (hand1: char list) (hand2: char list): int =
    let comparedByCard = 
        hand1 |> 
        List.indexed |> 
        List.map (fun (idx, h1) -> (h1 |> cardValue jForJoker) - (hand2[idx] |> cardValue jForJoker))

    if comparedByCard |> List.filter (fun x -> x <> 0) |> List.length = 0 then 0 // equivalent hands
    else comparedByCard |> List.skipWhile (fun x -> x = 0) |> List.head |> (fun x -> if x < 0 then -1 else 1)
    

let winnerSort (jForJoker: bool) (hand1: Hand) (hand2: Hand): int = 
    let adjustForJoker (cards: char list) =
        if (not jForJoker) then cards 
        else
            cards |> List.map (fun card -> 
                if card = 'J' 
                then 
                    let nonJ = cards |> sortByOccurrences |> List.filter (fun (card, _) -> card <> 'J')
                    if nonJ |> List.isEmpty then 'A' else (nonJ.Head |> fst)
                else card)

    let hand1Type = hand1.Cards |> adjustForJoker |> sortByOccurrences |> parseHandType
    let hand2Type = hand2.Cards |> adjustForJoker |> sortByOccurrences |> parseHandType

    if hand1Type = hand2Type then tieBreaker jForJoker hand1.Cards hand2.Cards
        else if hand1Type > hand2Type then 1 
        else -1

let parsedHands = 
    input |> List.map (fun line -> line.Split(" ") |> (fun parts -> 
        { 
            Cards = parts[0].ToCharArray() |> List.ofArray;
            Str = parts[0];
            Bid = parts.[1] |> int;
        }))
    
let part1 = 
    parsedHands
    |> List.sortWith (winnerSort false)
    |> List.indexed |> List.map (fun (idx, hand) -> (idx + 1) * hand.Bid)
    |> List.sum

let part2 =
    parsedHands
    |> List.sortWith (winnerSort true)
    |> List.indexed |> List.map (fun (idx, hand) -> (idx + 1) * hand.Bid)
    |> List.sum
