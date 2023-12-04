use std::collections::HashSet;

fn main() {
    // part 1
    let inputs = include_str!("inputs/04");
    let lines: Vec<&str> = inputs.split('\n').collect();
    let mut winning_nums = vec![HashSet::<i64>::new(); lines.len()];
    let mut my_scores = vec![0; lines.len()];

    // for p2
    let mut winning_amounts = vec![0; lines.len()];

    for (i, line) in lines.iter().enumerate() {
        let s = line.split_once(':').unwrap();
        let (win, mine) = s.1.split_once('|').unwrap();
        for n in win.split_ascii_whitespace() {
            winning_nums[i].insert(n.parse::<i64>().unwrap());
        }
        let mut score_for_card = 0;
        let mut wins = 0;
        for n in mine.split_ascii_whitespace() {
            if winning_nums[i].contains(&n.parse::<i64>().unwrap()) {
                if score_for_card == 0 {
                    score_for_card = 1;
                } else {
                    score_for_card <<= 1;
                }
                wins += 1;
            }
        }
        my_scores[i] = score_for_card;
        winning_amounts[i] = wins;
    }

    let part_1: i64 = my_scores.iter().sum();
    println!("part1: {}", part_1);

    // part 2;
    let mut scratch_cards = vec![1_i64; lines.len()];

    for (i, _) in lines.iter().enumerate() {
        let win_amount = winning_amounts[i];
        let end = i + win_amount as usize;

        let num_wins = scratch_cards[i];
        for (j, card_num) in scratch_cards
            .iter_mut()
            .enumerate()
            .take(end + 1)
            .skip(i + 1)
        {
            if j < lines.len() {
                *card_num += num_wins;
            }
        }
    }

    let part2: i64 = scratch_cards.iter().sum();
    println!("part2: {}", part2);
}
