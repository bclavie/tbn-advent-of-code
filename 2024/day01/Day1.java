package tbn.aoc2024;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class Day1 {

    private record Lists(List<Integer> left, List<Integer> right) {
    }

    public static Lists getLists() {
        List<String> strings = readFile("Day1.txt");
        List<Integer> left = new ArrayList<>();
        List<Integer> right = new ArrayList<>();

        for (String string : strings) {
            String[] split = string.split(" ");
            left.add(Integer.valueOf(split[0]));
            right.add(Integer.valueOf(split[split.length - 1]));
        }

        left.sort(Integer::compareTo);
        right.sort(Integer::compareTo);

        return new Lists(left, right);
    }

    public static void part1() {
        Lists lists = getLists();
        List<Integer> left = lists.left;
        List<Integer> right = lists.right;

        int result = 0;

        for (int i = 0; i < left.size(); i++) {
            result += Math.abs(left.get(i) - right.get(i));
        }

        System.out.println(result);

    }

    public static void part2() {
        Lists lists = getLists();
        List<Integer> left = lists.left;
        List<Integer> right = lists.right;

        int result = 0;

        for (Integer l : left) {
            int nFound = 0;

            for (int i = 0; i < right.size(); i++) {
                if (l.equals(right.get(i))) {
                    nFound++;
                }
            }

            result += nFound * l;
        }

        System.out.println(result);
    }

    public static void main(String[] args) {
        part1();
        part2();
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
