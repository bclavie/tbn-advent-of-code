fn main() {
    let inputs = include_str!("inputs/09");

    let sequences: Vec<Vec<i64>> = inputs
        .lines()
        .map(|line| line.split(' ').map(|s| s.parse::<i64>().unwrap()).collect())
        .collect();

    let sum_extrapolated_seqeunces: i64 = sequences.iter().map(|s| extrapolate_sequence(s)).sum();

    // part1
    println!("part1: {}", sum_extrapolated_seqeunces);

    let sum_extrapolated_sequences_backward: i64 = sequences
        .iter()
        .map(|s| extrapolate_sequence_backward(s))
        .sum();
    println!("part2: {}", sum_extrapolated_sequences_backward);
}

fn extrapolate_sequence(seq: &[i64]) -> i64 {
    let seq_diffs: Vec<i64> = seq.windows(2).map(|win| win[1] - win[0]).collect();
    if seq_diffs.iter().all(|i| *i == 0) {
        // this means all elements on seq are the same, so just return the first element.
        seq[0]
    } else {
        let diff = extrapolate_sequence(&seq_diffs);
        seq.last().unwrap() + diff
    }
}

fn extrapolate_sequence_backward(seq: &[i64]) -> i64 {
    let seq_diffs: Vec<i64> = seq.windows(2).map(|win| win[1] - win[0]).collect();
    if seq_diffs.iter().all(|i| *i == 0) {
        // this means all elements on seq are the same, so just return the first element.
        seq[0]
    } else {
        let diff = extrapolate_sequence_backward(&seq_diffs);
        seq.first().unwrap() - diff
    }
}
