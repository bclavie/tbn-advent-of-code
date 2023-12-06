package tbn.aoc2023;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day6 {
    public static final Pattern numberPattern = Pattern.compile("\\d+");

    public static void main(String[] args) throws FileNotFoundException {
        part1();
        part2();
    }

    public static void part1() throws FileNotFoundException {
        List<String> list = readFile("Day6.txt");
        String timeString = list.get(0);

        String distanceString = list.get(1);

        List<Race> races = getRaces(getIntsFromLine(timeString), getIntsFromLine(distanceString));
        int score = 1;
        for(Race r : races) {
            score *= getWinningPressesForRace(r).size();
        }

        System.out.println(score);
    }

    public static void part2() {
        //Time:      53897698
        //Distance:  313109012141201
        Race r = new Race(313109012141201L, 53897698);
        List<Integer> winningPressesForRace = getWinningPressesForRace(r);
        System.out.println(winningPressesForRace.size());
    }

    public static List<Integer> getWinningPressesForRace(Race race) {
        List<Integer> winners = new ArrayList<>();
        for (int i = 1; i < race.time; i++) {
            if (checkIfPressWinsRace(i, race)) {
                winners.add(i);
            }
        }

        return winners;
    }

    public static boolean checkIfPressWinsRace(long press, Race race) {
        long distance = press * (race.time - press);
        return distance > race.distance;
    }

    public static List<Race> getRaces(List<Integer> times, List<Integer> distances) {
        List<Race> races = new ArrayList<>();
        for (int i = 0; i < times.size(); i++) {
            races.add(new Race(distances.get(i), times.get(i)));
        }
        return races;
    }

    public static List<Integer> getIntsFromLine(String line) {
        List<Integer> result = new ArrayList<>();
        Matcher matcher = numberPattern.matcher(line);
        int i = 0;
        while (matcher.find(i)) {
            result.add(Integer.valueOf(matcher.group()));
            i = matcher.end();
        }

        return result;

    }

    public static class Race {
        long distance;
        long time;

        public Race(long distance, long time) {
            this.distance = distance;
            this.time = time;
        }

        @Override
        public String toString() {
            return "Race{" +
                    "distance=" + distance +
                    ", time=" + time +
                    '}';
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
