package tbn.aoc2023;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day9 {
    public static final Pattern numberPattern = Pattern.compile("-?\\d+");

    public static void main(String[] args) throws FileNotFoundException {
        //part1();
        part2();
    }

    public static void part1() throws FileNotFoundException {
        List<String> lines = readFile("Day9.txt");
        List<List<Integer>> sequences = getSequences(lines);

        int sum = 0;
        for (List<Integer> sequence : sequences) {
            List<List<Integer>> subsequences = generateSubsequences(sequence);
            int i = generateNextNumberInSequence(sequence, subsequences);
            sum += i;
        }

        System.out.println(sum);

    }

    public static void part2() throws FileNotFoundException {
        List<String> lines = readFile("Day9.txt");
        List<List<Integer>> sequences = getSequences(lines);

        int sum = 0;
        for (List<Integer> sequence : sequences) {
            List<List<Integer>> subsequences = generateSubsequences(sequence);
            int i = generatePreviousNumberInSequence(sequence, subsequences);
            sum += i;
        }

        System.out.println(sum);
    }

    public static int generateNextNumberInSequence(List<Integer> origin, List<List<Integer>> subsequences) {
        for (int i = subsequences.size() - 1; i - 1 >= 0; i--) {
            Integer nextNumber = generateNextNumber(subsequences.get(i), subsequences.get(i - 1));
            subsequences.get(i - 1).add(nextNumber);
        }

        return generateNextNumber(origin, subsequences.get(0));
    }

    public static int generatePreviousNumberInSequence(List<Integer> origin, List<List<Integer>> subsequences) {
        for (int i = subsequences.size() - 1; i - 1 >= 0; i--) {
            Integer nextNumber = generatePreviousNumber(subsequences.get(i), subsequences.get(i - 1));
            subsequences.get(i - 1).add(0, nextNumber);
        }

        return generatePreviousNumber(subsequences.get(0), origin);
    }

    public static Integer generateNextNumber(List<Integer> a, List<Integer> b) {
        return a.get(a.size() - 1) + b.get(b.size() - 1);
    }

    public static Integer generatePreviousNumber(List<Integer> a, List<Integer> b) {
        return b.get(0) - a.get(0);
    }

    public static List<List<Integer>> generateSubsequences(List<Integer> origin) {
        List<List<Integer>> result = new ArrayList<>();
        List<Integer> currentSequence = origin;
        while (!currentSequence.stream().allMatch(i -> i == 0)) {
            List<Integer> subsequence = generateSubsequence(currentSequence);
            result.add(subsequence);
            currentSequence = subsequence;
        }
        return result;
    }


    public static List<Integer> generateSubsequence(List<Integer> origin) {
        List<Integer> result = new ArrayList<>();

        for (int i = 0; i + 1 < origin.size(); i++) {
            result.add(origin.get(i + 1) - origin.get(i));
        }
        return result;
    }

    public static List<List<Integer>> getSequences(List<String> lines) {
        return lines.stream().map(Day9::getSequenceFromLine).toList();
    }

    public static List<Integer> getSequenceFromLine(String line) {
        Matcher matcher = numberPattern.matcher(line);
        List<Integer> result = new ArrayList<>();
        while (matcher.find()) {
            result.add(Integer.valueOf(matcher.group()));
        }

        return result;
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
