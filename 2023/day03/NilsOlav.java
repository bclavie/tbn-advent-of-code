package tbn.aoc2023;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day3 {

    public static final Pattern partPattern = Pattern.compile("\\d+");

    public static final Pattern characterPattern = Pattern.compile("[^\\d.]");

    public static final Pattern gearPattern = Pattern.compile("\\*");

    public static void main(String[] args) throws FileNotFoundException {
        part1();
        part2();
    }

    public static void part2() throws FileNotFoundException {
        List<String> lines = readFile("Day3.txt");
        List<Coordinate> potentialGears = getPotentialGears(lines);
        List<Part> parts = getParts(lines);
        int gearScore = calculateGearScore(potentialGears, parts);
        System.out.println(gearScore);
    }

    public static void part1() throws FileNotFoundException {
        List<String> lines = readFile("Day3.txt");
        List<Part> parts = getParts(lines);
        List<Coordinate> symbols = getSymbols(lines);
        List<Part> partsInUse = getPartsInUse(parts, symbols);
        int sum = 0;
        for (Part r : partsInUse) {
            sum += r.number;
        }
        System.out.println(sum);
    }

    public static int calculateGearScore(List<Coordinate> potentialGears, List<Part> parts) {
        int sum = 0;
        for (Coordinate gear : potentialGears) {
            sum += calculateGearScore(gear, parts);
        }
        return sum;
    }

    public static int calculateGearScore(Coordinate potentialGear, List<Part> parts) {
        List<Part> matchingParts = parts.stream().filter(p -> partMatch(p, potentialGear)).toList();
        if (matchingParts.size() == 2) {
            return matchingParts.get(0).number * matchingParts.get(1).number;
        } else {
            return 0;
        }
    }

    public static List<Part> getPartsInUse(List<Part> potential, List<Coordinate> symbols) {
        List<Part> result = new ArrayList<>();
        for (Part r : potential) {
            if (partMatch(r, symbols)) {
                result.add(r);
            }
        }
        return result;
    }

    public static boolean partMatch(Part part, List<Coordinate> symbols) {
        for (Coordinate c : symbols) {
            if (partMatch(part, c)) {
                return true;
            }
        }
        return false;
    }

    public static boolean partMatch(Part part, Coordinate c) {
        int maxX = part.maxX + 1;
        int minX = part.minX - 1;
        int maxY = part.y + 1;
        int minY = part.y - 1;
        if (c.x <= maxX && c.x >= minX) {
            return c.y <= maxY && c.y >= minY;
        }
        return false;
    }

    public static List<Coordinate> getPotentialGears(List<String> lines) {
        List<Coordinate> result = new ArrayList<>();
        for (int i = 0; i < lines.size(); i++) {
            String line = lines.get(i);
            Matcher matcher = gearPattern.matcher(line);
            int currentIndex = 0;
            while (matcher.find(currentIndex)) {
                result.add(new Coordinate(matcher.start(), i));
                currentIndex = matcher.end();
            }
        }

        return result;
    }

    public static List<Coordinate> getSymbols(List<String> lines) {
        List<Coordinate> result = new ArrayList<>();
        for (int i = 0; i < lines.size(); i++) {
            String line = lines.get(i);
            Matcher matcher = characterPattern.matcher(line);
            int currentIndex = 0;
            while (matcher.find(currentIndex)) {
                result.add(new Coordinate(matcher.start(), i));
                currentIndex = matcher.end();
            }
        }

        return result;
    }

    public static List<Part> getParts(List<String> lines) {
        List<Part> result = new ArrayList<>();
        for (int i = 0; i < lines.size(); i++) {
            String line = lines.get(i);
            Matcher matcher = partPattern.matcher(line);
            int currentIndex = 0;
            while (matcher.find(currentIndex)) {
                Part part = new Part();
                part.y = i;
                part.minX = matcher.start();
                part.maxX = matcher.end() - 1;
                part.number = Integer.parseInt(line.substring(matcher.start(), matcher.end()));
                result.add(part);
                currentIndex = matcher.end();
            }
        }
        return result;
    }

    public static class Part {
        public int minX;
        public int maxX;
        public int y;

        public int number;

        @Override
        public String toString() {
            return "Part{" +
                    "minX=" + minX +
                    ", maxX=" + maxX +
                    ", y=" + y +
                    ", number=" + number +
                    '}';
        }
    }

    public static class Coordinate {
        public int x;
        public int y;

        public Coordinate(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public String toString() {
            return "Coordinate{" +
                    "x=" + x +
                    ", y=" + y +
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
