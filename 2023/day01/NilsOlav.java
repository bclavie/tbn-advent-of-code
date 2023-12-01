package tbn.aoc2023;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day1 {

    public static final Pattern numberPattern = Pattern.compile("[0-9]");
    public static final Pattern wordPattern = Pattern.compile("one|two|three|four|five|six|seven|eight|nine|zero");
    public static List<String> numberWords = List.of("zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine");

    public static void main(String[] args) throws FileNotFoundException {
        //solvePart1();
        //56049
        solvePart2();
    }

    public static void solvePart1() throws FileNotFoundException {
        List<String> lines = readFile("Day1.txt");
        int sum = 0;
        for (String s : lines) {
            sum += getNumberFromLine(s);
        }

        System.out.println("Part1: " + sum);
    }

    public static void solvePart2() throws FileNotFoundException {
        List<String> lines = readFile("Day1.txt");
        int sum = 0;
        for (String s : lines) {
            sum += getNumberFromLineWithWords(s);
        }
        System.out.println(sum);
    }

    public static int getNumberFromLineWithWords(String line) {
        int firstWordIndex = getFirstIndexOfNumberWord(line);
        int firstNumIndex = getFirstIndexOfNumber(line);
        if (firstNumIndex > line.length()) {
            int lastWordIndex = getLastIndexOfNumberWord(line, firstWordIndex);
            return Integer.parseInt("" + parseNumberWordFromIndex(line, firstWordIndex) + parseNumberWordFromIndex(line, lastWordIndex));
        } else if (firstWordIndex > line.length()) {
            int lastNumIndex = getLastIndexOfNumber(line, firstNumIndex);
            return Integer.parseInt("" + line.charAt(firstNumIndex) + line.charAt(lastNumIndex));
        } else {
            int lastWordIndex = getLastIndexOfNumberWord(line, firstWordIndex);
            int lastNumIndex = getLastIndexOfNumber(line, firstNumIndex);

            int firstNum = -1;
            if (firstNumIndex < firstWordIndex) {
                firstNum = Integer.parseInt("" + line.charAt(firstNumIndex));
            } else {
                firstNum = parseNumberWordFromIndex(line, firstWordIndex);
            }

            int lastNum = -1;
            if (lastNumIndex > lastWordIndex) {
                lastNum = Integer.parseInt("" + line.charAt(lastNumIndex));
            } else {
                lastNum = parseNumberWordFromIndex(line, lastWordIndex);
            }

            return Integer.parseInt("" + firstNum + lastNum);

        }
    }

    public static int parseNumberWordFromIndex(String line, int start) {
        int i = 0;
        int number = -1;
        do {
            number = numberWords.indexOf(line.substring(start, start + i));
            i++;
        } while (number < 0);
        return number;
    }

    public static int getNumberFromLine(String line) {
        int start = getFirstIndexOfNumber(line);
        int end = getLastIndexOfNumber(line, start);
        String number = "" + line.charAt(start) + line.charAt(end);
        return Integer.parseInt(number);
    }

    public static int getFirstIndexOfNumber(String line) {
        Matcher matcher = numberPattern.matcher(line);
        if (matcher.find()) {
            return matcher.start();
        } else {
            return Integer.MAX_VALUE;
        }
    }

    public static int getLastIndexOfNumber(String line, int startIndex) {
        Matcher matcher = numberPattern.matcher(line);
        int currentLast = startIndex;
        while (matcher.find(currentLast + 1)) {
            currentLast = matcher.start();
        }

        return currentLast;
    }

    public static int getFirstIndexOfNumberWord(String line) {
        Matcher matcher = wordPattern.matcher(line);
        if (matcher.find()) {
            return matcher.start();
        } else {
            return Integer.MAX_VALUE;
        }
    }

    public static int getLastIndexOfNumberWord(String line, int startIndex) {
        Matcher matcher = wordPattern.matcher(line);
        int currentLast = startIndex;
        while (matcher.find(currentLast + 1)) {
            currentLast = matcher.start();
        }

        return currentLast;
    }


    public static List<String> readFile(String fileName) throws FileNotFoundException {
        File myObj = new File("src/main/resources/" + fileName);
        Scanner myReader = new Scanner(myObj);

        List<String> lines = new ArrayList<>();

        while (myReader.hasNext()) {
            lines.add(myReader.next());
        }

        return lines;

    }
}
