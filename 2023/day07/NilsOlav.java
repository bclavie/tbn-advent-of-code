package tbn.aoc2023;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;


public class Day7 {

    public static String ranks = "AKQJT98765432";

    public static String ranks2 = "AKQT98765432J";

    public static boolean part2 = false;

    public static void main(String[] args) throws FileNotFoundException {
        compute();
        part2 = true;
        System.out.println("###############");
        compute();
    }

    public static void compute() throws FileNotFoundException {
        List<String> lines = readFile("Day7.txt");
        List<Hand> hands = getHandsFromLines(lines);

        hands.sort(Comparator.naturalOrder());
        int sum = 0;
        for (int i = 0; i < hands.size(); i++) {
            System.out.println(hands.get(i));
            sum += hands.get(i).bet * (i + 1);
        }

        System.out.println(sum);
    }


    public static List<Hand> getHandsFromLines(List<String> lines) {
        return new ArrayList<>(lines.stream().map(Day7::getHandFromLine).toList());
    }

    //Lower is better
    public static int getHandType(Hand hand) {
        List<String> cards = new ArrayList<>(hand.cards);
        Set<String> uniqueCards = new HashSet<>();
        uniqueCards.addAll(cards);
        if (uniqueCards.size() == 1) { //5 of a kind
            return uniqueCards.size();
        } else if (uniqueCards.size() >= 4) { //1 pair or high card
            return uniqueCards.size() + 2;
        } else if (uniqueCards.size() == 2) { //Full house or 4 of a kind;
            cards.removeAll(List.of(cards.get(0)));
            if (cards.size() == 4 || cards.size() == 1) { //4 of a kind
                return 2;
            } else { //Full house
                return 3;
            }
        } else { // 3 of a kind or two pair
            List<List<String>> result = new ArrayList<>();
            for (String u : uniqueCards) {
                result.add(cards.stream().filter(c -> c.equals(u)).toList());
            }

            if (result.stream().anyMatch(l -> l.size() == 3)) { //3 of a kind
                return 4;
            } else { //two pair
                return 5;
            }
        }
    }

    //Lower is better
    public static int getHandType2(Hand hand) {
        if (!hand.cards.contains("J")) {
            return getHandType(hand);
        }
        List<String> cards = new ArrayList<>(hand.cards);
        cards.removeAll(List.of("J"));
        int numberOfJokers = 5 - cards.size();
        if (numberOfJokers == 5) {
            return 1;
        }
        HashMap<String, Counter> typeCount = new HashMap<>();
        for (String c : cards) {
            typeCount.put(c, new Counter());
        }

        for (String c : cards) {
            typeCount.get(c).inc();
        }

        List<Integer> counts = new ArrayList<>(typeCount.values().stream().map(c -> c.value).toList());
        counts.sort(Comparator.reverseOrder());
        int largest = counts.get(0);
        if (largest + numberOfJokers == 5) { //5 of a kind
            return 1;
        } else if (largest + numberOfJokers == 4) { // 4 of a kind
            return 2;
        } else if (largest + numberOfJokers == 3) {// 3 of a kind or full house
            if (counts.get(1) == 2) {
                return 3;
            } else {
                return 4;
            }
        } else {
            return 6;
        }


    }

    public static Hand getHandFromLine(String line) {
        String[] split = line.split(" ");

        List<String> cards = new ArrayList<>();

        for (int i = 0; i < split[0].length(); i++) {
            cards.add(String.valueOf(split[0].charAt(i)));
        }

        return new Hand(cards, Integer.parseInt(split[1]));
    }

    public static class Counter {
        public int value = 0;

        public void inc() {
            value++;
        }

        @Override
        public String toString() {
            return String.valueOf(value);
        }
    }

    public static class Hand implements Comparable<Hand> {
        public List<String> cards;
        public int bet;

        public int handType;

        public Hand(List<String> cards, int bet) {
            this.cards = cards;
            this.bet = bet;
            if (part2) {
                this.handType = getHandType2(this);
            } else {
                this.handType = getHandType(this);
            }
        }

        @Override
        public String toString() {
            return "H{" +
                    "cards=" + cards +
                    ", bet=" + bet +
                    ", type=" + handType +
                    '}';
        }

        @Override
        public int compareTo(Hand hand) {
            if (handType < hand.handType) {
                return 1;
            } else if (handType > hand.handType) {
                return -1;
            } else {
                for (int i = 0; i < cards.size(); i++) {
                    if (!cards.get(i).equals(hand.cards.get(i))) {
                        return compareRank(cards.get(i), hand.cards.get(i));
                    }
                }
                return 0;
            }
        }

        private int compareRank(String c1, String c2) {
            if (part2) {
                return -(ranks2.indexOf(c1) - ranks2.indexOf(c2));
            } else {
                return -(ranks.indexOf(c1) - ranks.indexOf(c2));
            }
        }
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
