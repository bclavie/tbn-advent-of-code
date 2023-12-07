import math


def entropy(hand):
    probabilities = [hand.count(card) / len(hand) for card in set(hand)]
    return -sum([prob * math.log(prob) for prob in probabilities])


def part_2_entropy(hand):
    if "J" * 5 == hand:
        return entropy(hand)
    top = sorted(
        hand.replace("J", ""),
        key=lambda card: hand.count(card),
    )[-1]
    return entropy(hand.replace("J", top))


def card_strength(card, part_2=False):
    idx = "AKQJT98765432"
    if part_2:
        idx = idx.replace("J", "") + "J"
    return idx.index(card)


def rank_hands(hands, part_2=False):
    return sorted(
        hands,
        key=lambda hand: (
            entropy(hand[0]) if not part_2 else part_2_entropy(hand[0]),
            *map(card_strength, hand[0], [part_2] * len(hand[0])),
        ),
    )


if __name__ == "__main__":
    hands_input = [
        (x.split(" ")[0], int(x.split(" ")[1]))
        for x in open("input.txt").read().splitlines()
    ]

    ranked_hands = rank_hands(hands_input)

    total_winnings = sum(
        bid * (len(hands_input) - rank) for rank, (hand, bid) in enumerate(ranked_hands)
    )
    print("Part 1 Total winnings:", total_winnings)

    ranked_hands_p2 = rank_hands(hands_input, part_2=True)

    total_winnings_p2 = sum(
        bid * (len(hands_input) - rank)
        for rank, (hand, bid) in enumerate(ranked_hands_p2)
    )
    print("Part 2 Total winnings:", total_winnings_p2)
