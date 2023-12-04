package tbn.aoc2023;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day4 {

    public static final Pattern numberPattern = Pattern.compile("\\d+");

    public static void main(String[] args) throws FileNotFoundException {
        part1();
        part2();
    }

    private static void part1() throws FileNotFoundException {
        List<String> lines = readFile("Day4.txt");
        List<Card> cards = getCardsFromLines(lines);
        int sum = 0;
        for (Card c : cards) {
            int scoreFromCard = getScoreFromCard(c);
            sum += scoreFromCard;
        }
        System.out.println(sum);

    }

    private static void part2() throws FileNotFoundException {
        List<String> lines = readFile("Day4.txt");
        List<Card> cards = getCardsFromLines(lines);
        Map<Integer, Counter> numberOfCard = new HashMap<>();
        for (Card c : cards) {
            numberOfCard.put(c.id, new Counter());
        }
        for (Card c : cards) {
            int amountOfCard = numberOfCard.get(c.id).value;
            int numberOfMatchesFromCard = getNumberOfMatchesFromCard(c);
            for (int i = c.id + 1; i < c.id + numberOfMatchesFromCard + 1; i++) {
                numberOfCard.get(i).inc(amountOfCard);
            }
        }

        Integer integer = numberOfCard.values().stream().map(c -> c.value).reduce(Integer::sum).get();
        System.out.println(integer);

    }

    public static int getScoreFromCard(Card card) {
        int matches = getNumberOfMatchesFromCard(card);
        if (matches == 0) {
            return 0;
        } else {
            return (int) Math.pow(2, matches - 1);
        }

    }

    public static int getNumberOfMatchesFromCard(Card card) {
        List<Integer> matches = card.winningNumbers.stream().filter(n -> card.drawnNumbers.contains(n)).toList();
        return matches.size();
    }


    public static List<Card> getCardsFromLines(List<String> lines) {
        return lines.stream().map(Day4::getCardFromLine).toList();
    }

    public static Card getCardFromLine(String line) {
        Card card = new Card();
        String[] split = line.split("\\|");
        String winners = split[0];
        String drawn = split[1];
        Matcher winnerMatcher = numberPattern.matcher(winners);
        winnerMatcher.find(); //Find card number
        card.id = Integer.parseInt(winners.substring(winnerMatcher.start(), winnerMatcher.end()));
        int index = winnerMatcher.end();
        while (winnerMatcher.find(index)) {
            card.winningNumbers.add(Integer.valueOf(winners.substring(winnerMatcher.start(), winnerMatcher.end())));
            index = winnerMatcher.end();
        }
        index = 0;
        Matcher drawnMatcher = numberPattern.matcher(drawn);
        while (drawnMatcher.find(index)) {
            card.drawnNumbers.add(Integer.valueOf(drawn.substring(drawnMatcher.start(), drawnMatcher.end())));
            index = drawnMatcher.end();
        }
        return card;
    }

    public static class Card {

        public int id;
        public Set<Integer> winningNumbers = new HashSet<>();
        public Set<Integer> drawnNumbers = new HashSet<>();

        @Override
        public String toString() {
            return "Card{" +
                    "id=" + id +
                    ", winningNumbers=" + winningNumbers +
                    ", drawnNumbers=" + drawnNumbers +
                    '}';
        }
    }

    public static class Counter {
        public int value = 1;

        public void inc(int v) {
            value += v;
        }

        @Override
        public String toString() {
            return String.valueOf(value);
        }
    }

    public static void printArray(String[] arr) {
        StringJoiner sj = new StringJoiner("','", "['", "']");
        for (String s : arr) {
            sj.add(s);
        }
        System.out.println(sj);
    }


    public static List<String> readFile(String fileName) throws FileNotFoundException {
        File myObj = new File("src/main/resources/" + fileName);
        Scanner myReader = new Scanner(myObj);

        List<String> lines = new ArrayList<>();

        while (myReader.hasNextLine()) {
            String next = myReader.nextLine();
            lines.add(next);
        }

        return lines;
    }
}
