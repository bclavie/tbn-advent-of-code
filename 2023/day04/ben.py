from collections import defaultdict

cards = [x.strip() for x in open("input.txt", "r").readlines()]
# init p1
part_1_score = 0
# init p2
card_copies = defaultdict(int)

for current_card_idx, card in enumerate(cards):
    card_copies[current_card_idx] += 1
    winners, hand = (
        set(int(x) for x in card_group.split())
        for card_group in card.split(": ")[1].split(" | ")
    )

    part_1_score += 2 ** (len(winners & hand) - 1) if winners & hand else 0

    for future_card_idx in range(1, len(winners & hand) + 1):
        card_copies[current_card_idx + future_card_idx] += card_copies[current_card_idx]

print(f"Part 1: {part_1_score}")
print(f"Part 2: {sum(card_copies.values())}")
