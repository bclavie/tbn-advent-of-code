package tbn.aoc2024;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

public class Day4 {
    private record Point(int x, int y) {
    }

    public static void main(String[] args) {
        part1();
        part2();
    }

    public static void part1() {
        char[][] matrix = getMatrix();

        int sum = 0;
        for (int x = 0; x < matrix[0].length; x++) {
            for (int y = 0; y < matrix.length; y++) {
                int matches = findMatches(matrix, new Point(y, x), "XMAS");
                sum += matches;
            }
        }

        System.out.println(sum);
    }

    public static void part2() {
        char[][] matrix = getMatrix();

        int sum = 0;
        for (int x = 0; x < matrix[0].length; x++) {
            for (int y = 0; y < matrix.length; y++) {
                if(checkXmas(matrix, new Point(y, x))) {
                    sum++;
                }
            }
        }
        System.out.println(sum);
    }

    private static boolean checkXmas(char[][] matrix, Point p) {
        try {
            if (matrix[p.y][p.x] == 'A') {
                return (Character.getNumericValue(matrix[p.y - 1][p.x - 1]) + Character.getNumericValue(matrix[p.y + 1][p.x + 1]) == 50)
                        && (Character.getNumericValue(matrix[p.y + 1][p.x - 1]) + Character.getNumericValue(matrix[p.y - 1][p.x + 1]) == 50);
            }
        } catch (ArrayIndexOutOfBoundsException e) {
            return false;
        }
        return false;
    }

    private static char[][] getMatrix() {
        List<String> lines = readFile("Day4.txt");
        char[][] matrix = new char[lines.getFirst().length()][lines.size()];
        for (int i = 0; i < lines.size(); i++) {
            String line = lines.get(i);
            for (int j = 0; j < line.length(); j++) {
                matrix[j][i] = line.charAt(j);
            }
        }

        return matrix;
    }

    private static int findMatches(char[][] matrix, Point p, String word) {
        Optional<Integer> reduce = getVectors().stream().map(v -> checkWord(matrix, p, v, word)).reduce(Integer::sum);
        return reduce.get();
    }

    public static int checkWord(char[][] matrix, Point p, Point v, String word) {
        try {
            if (word.charAt(0) == matrix[p.y][p.x]) {
                if (word.length() == 1) {
                    return 1;
                } else {
                    String newWord = word.substring(1);
                    return checkWord(matrix, new Point(p.x + v.x, p.y + v.y), v, newWord);
                }
            } else {
                return 0;
            }
        } catch (ArrayIndexOutOfBoundsException e) {
            return 0;
        }
    }

    public static List<Point> getVectors() {
        return List.of(
                new Point(1, 1),
                new Point(1, 0),
                new Point(0, 1),
                new Point(-1, -1),
                new Point(-1, 0),
                new Point(0, -1),
                new Point(1, -1),
                new Point(-1, 1)
        );
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
