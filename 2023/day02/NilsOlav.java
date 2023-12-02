package tbn.aoc2023;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;
import java.util.regex.Pattern;

public class Day2 {

    public static final Pattern numberPattern = Pattern.compile("[0-9]");


    public static void main(String[] args) throws FileNotFoundException {
        part1();
        part2();
    }

    public static void part1() throws FileNotFoundException {
        List<String> lines = readFile("Day2.txt");
        HashMap<String, Integer> cubesInBag = new HashMap<>();
        cubesInBag.put("red", 12);
        cubesInBag.put("green", 13);
        cubesInBag.put("blue", 14);

        int sum = 0;
        for (String line : lines) {
            Game game = getGameFromLine(line);
            boolean valid = validateGame(game, cubesInBag);
            if (valid) {
                sum += game.id;
            }
        }
        System.out.println(sum);
    }

    public static void part2() throws FileNotFoundException {
        List<String> lines = readFile("Day2.txt");

        int sum = 0;
        for (String line : lines) {
            Game game = getGameFromLine(line);
            int power = getPowerOfGame(game);
            sum += power;
        }
        System.out.println(sum);
    }

    public static boolean validateGame(Game game, Map<String, Integer> cubesInBag) {
        return game.cubeDraws.stream().allMatch(draw -> validateDraw(draw, cubesInBag));
    }

    public static boolean validateDraw(Map<String, Integer> draw, Map<String, Integer> cubesInBag) {
        for (String color : draw.keySet()) {
            if (draw.get(color) > cubesInBag.get(color)) {
                return false;
            }
        }
        return true;
    }

    public static int getPowerOfGame(Game game) {
        Map<String, Integer> minimumCubes = getMinimumCubes(game);
        int result = 1;
        for (int i : minimumCubes.values()) {
            result *= i;
        }

        return result;
    }

    public static Map<String, Integer> getMinimumCubes(Game game) {
        Map<String, Integer> result = new HashMap<>();
        for (Map<String, Integer> draw : game.cubeDraws) {
            for (String colour : draw.keySet()) {
                if (!result.containsKey(colour) || result.get(colour) < draw.get(colour)) {
                    result.put(colour, draw.get(colour));
                }
            }
        }

        return result;
    }

    public static Game getGameFromLine(String line) {
        Game game = new Game();
        game.id = getGameId(line);
        String[] draws = line.split(":")[1].split(";");
        for (String draw : draws) {
            Map<String, Integer> drawMap = getDraw(draw);
            game.cubeDraws.add(drawMap);
        }
        return game;
    }

    public static Map<String, Integer> getDraw(String draw) {
        Map<String, Integer> result = new HashMap<>();
        String[] countAndColours = draw.split(",");
        for (String s : countAndColours) {
            //Get count and colour
            String[] countAndColour = s.split(" ");
            result.put(countAndColour[2], Integer.valueOf(countAndColour[1]));
        }
        return result;
    }

    public static int getGameId(String line) {
        return Integer.parseInt(line.split(":")[0].split(" ")[1]);
    }


    public static class Game {
        public int id;
        public List<Map<String, Integer>> cubeDraws = new ArrayList<>();

        @Override
        public String toString() {
            return "Game{" +
                    "id=" + id +
                    ", cubeDraws=" + cubeDraws +
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

    public static void printArray(String[] arr) {
        StringJoiner sj = new StringJoiner("','", "['", "']");
        for (String s : arr) {
            sj.add(s);
        }
        System.out.println(sj);
    }
}
