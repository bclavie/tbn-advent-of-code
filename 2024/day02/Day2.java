package tbn.aoc2024;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;

public class Day2 {


    public static void main(String[] args) {
        part1();
        part2();
    }

    private static void part1() {
        List<String> strings = readFile("Day2.txt");
        List<List<Integer>> reports = getReports(strings);

        int result = 0;
        for (List<Integer> report : reports) {
            if (isReportSafe(report)) {
                result++;
            }
        }
        System.out.println(result);
    }

    private static void part2() {
        List<String> strings = readFile("Day2.txt");
        List<List<Integer>> reports = getReports(strings);

        int result = 0;
        for (List<Integer> report : reports) {
            if (isReportSafe(report)) {
                result++;
            } else if (doubleCheckReport(report)) {
                result++;
            }
        }

        System.out.println(result);

    }

    private static boolean doubleCheckReport(List<Integer> report) {
        for (int i = 0; i < report.size(); i++) {
            List<Integer> clone = new ArrayList<>(report);
            clone.remove(i);
            if (isReportSafe(clone)) {
                return true;
            }
        }

        return false;
    }

    private static List<List<Integer>> getReports(List<String> strings) {
        List<List<Integer>> result = new ArrayList<>();
        for (String string : strings) {
            String[] s = string.split(" ");
            result.add(Arrays.stream(s).map(Integer::valueOf).toList());
        }

        return result;
    }

    private static boolean isReportSafe(List<Integer> report) {
        return isReportSafeInc(report) || isReportSafeDec(report);
    }

    private static boolean isReportSafeInc(List<Integer> report) {
        for (int i = 0; i < report.size(); i++) {
            if (i + 1 == report.size()) {
                return true;
            }
            int diff = report.get(i) - report.get(i + 1);
            if (diff >= 0 || diff < -3) {
                return false;
            }
        }

        return true;
    }

    private static boolean isReportSafeDec(List<Integer> report) {
        for (int i = 0; i < report.size(); i++) {
            if (i + 1 == report.size()) {
                return true;
            }
            int diff = report.get(i) - report.get(i + 1);
            if (!((diff > 0) && (diff <= 3))) {
                return false;
            }
        }

        return true;
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
