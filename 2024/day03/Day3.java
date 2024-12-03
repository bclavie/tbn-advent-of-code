package tbn.aoc2024;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day3 {

    public static final Pattern productPattern = Pattern.compile("mul\\([0-9]{1,3},[0-9]{1,3}\\)");
    public static final Pattern productOrInstructPattern = Pattern.compile("(mul\\([0-9]{1,3},[0-9]{1,3}\\)|do\\(\\)|don't\\(\\))");

    public static void main(String[] args) {
        part1();
        part2();
    }

    public static void part1() {
        List<String> rows = readFile("Day3.txt");
        int sum = 0;
        for(String row : rows) {
            sum += getSumOfRow(row);
        }

        System.out.println(sum);
    }

    public static void part2() {
        List<String> rows = readFile("Day3.txt");
        int sum = 0;
        boolean useVal = true;
        for(String row : rows) {
            Matcher matcher = productOrInstructPattern.matcher(row);
            while (matcher.find()) {
                if(matcher.end() - matcher.start() == 4) {
                    useVal = true;
                } else if (matcher.end() - matcher.start() == 7) {
                    useVal = false;
                } else if (useVal) {
                    String mul = row.substring(matcher.start(), matcher.end());
                    int reduce = getMul(mul.substring(4, mul.length() - 1));
                    sum += reduce;
                }
            }
        }
        System.out.println(sum);

    }

    public static int getSumOfRow(String row) {
        int sum = 0;
        Matcher matcher = productPattern.matcher(row);
        while (matcher.find()) {
            String mul = row.substring(matcher.start(), matcher.end());
            int reduce = getMul(mul.substring(4, mul.length() - 1));
            sum += reduce;
        }

        return sum;
    }

    private static int getMul(String str) {
        return Arrays.stream(str.split(",")).map(Integer::valueOf).reduce(1, (acc, value) -> acc * value);
    }

    public static List<String> readFile(String fileName) {
        File myObj = new File("src/main/resources/" + fileName);
        Scanner myReader = null;
        try {
            myReader = new Scanner(myObj);
        } catch (FileNotFoundException e) {
            throw new RuntimeException(e);
        }

        List<String> lines = new ArrayList<>();

        while (myReader.hasNextLine()) {
            lines.add(myReader.nextLine());
        }

        return lines;

    }
}
