package tbn.aoc2024;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;

public class Day6 {
    private record Point(int x, int y) {
    }

    private enum Status {
        VISITED,
        EMPTY,
        OBSTACLE
    }

    private enum Direction {
        UP, RIGHT, DOWN, LEFT
    }

    public static void main(String[] args) {
        //part1();
        part2();
    }

    public static void part1() {
        Map<Point, Status> map = getMap();
        Point start = getStart(map);

        traverse(map, Direction.UP, start);

        int sum = 0;
        for (Map.Entry<Point, Status> entry : map.entrySet()) {
            if (entry.getValue() == Status.VISITED) {
                sum++;
            }
        }

        System.out.println(sum);
    }

    public static void part2() {
        Map<Point, Status> map = getMap();
        Point start = getStart(map);
        //Find all points where we will walk
        traverse(map, Direction.UP, start);

        List<Point> potentialBlocks = new ArrayList<>();
        for (Map.Entry<Point, Status> entry : map.entrySet()) {
            if (entry.getValue() == Status.VISITED) {
                potentialBlocks.add(entry.getKey());
            }
        }

        //We are not allowed to block the start
        potentialBlocks.remove(start);

        int sum = 0;
        //Test all these points if they cause a loop or not
        for (Point block : potentialBlocks) {
            Map<Point, Set<Direction>> visitMap = new HashMap<>();
            for (Point point : map.keySet()) {
                visitMap.put(point, new HashSet<>());
            }
            map = getMap();
            map.put(block, Status.OBSTACLE);
            if(traverseStopOnLoop(map, Direction.UP, start, visitMap)) {
                sum++;
            }
        }


        System.out.println(sum);

    }

    public static Point getStart(Map<Point, Status> map) {
        for (Map.Entry<Point, Status> entry : map.entrySet()) {
            if (entry.getValue() == Status.VISITED) {
                return entry.getKey();
            }
        }
        throw new RuntimeException("wat");
    }

    public static void traverse(Map<Point, Status> map, Direction d, Point p) {
        Point newP = getNextPoint(p, d);
        if (!map.containsKey(newP)) {
            return;
        }

        Status nextStatus = map.get(newP);
        if (nextStatus != Status.OBSTACLE) {
            map.put(newP, Status.VISITED);
            traverse(map, d, newP);
        } else {
            Direction newD = turn(d);
            traverse(map, newD, p);
        }

    }

    public static boolean traverseStopOnLoop(Map<Point, Status> map, Direction d, Point p, Map<Point, Set<Direction>> visitMap) {
        Point newP = getNextPoint(p, d);
        if (!map.containsKey(newP)) {
            return false;
        }

        if(visitMap.get(p).contains(d)) {
            return true;
        }

        visitMap.get(p).add(d);

        Status nextStatus = map.get(newP);
        if (nextStatus != Status.OBSTACLE) {
            map.put(newP, Status.VISITED);
            return traverseStopOnLoop(map, d, newP, visitMap);
        } else {
            Direction newD = turn(d);
            return traverseStopOnLoop(map, newD, p, visitMap);
        }
    }

    public static Direction turn(Direction d) {
        switch (d) {
            case UP:
                return Direction.RIGHT;
            case RIGHT:
                return Direction.DOWN;
            case DOWN:
                return Direction.LEFT;
            case LEFT:
                return Direction.UP;
        }

        throw new RuntimeException("wat");
    }

    public static Point getNextPoint(Point p, Direction d) {
        if (d == Direction.DOWN) {
            return new Point(p.x + 1, p.y);
        } else if (d == Direction.UP) {
            return new Point(p.x - 1, p.y);
        } else if (d == Direction.RIGHT) {
            return new Point(p.x, p.y + 1);
        } else {
            return new Point(p.x, p.y - 1);
        }
    }

    public static Map<Point, Status> getMap() {
        List<String> strings = readFile("Day6.txt");
        Map<Point, Status> map = new HashMap<>();
        for (int i = 0; i < strings.size(); i++) {
            String s = strings.get(i);
            for (int j = 0; j < s.length(); j++) {
                char c = s.charAt(j);
                Point p = new Point(i, j);
                if (c == '.') {
                    map.put(p, Status.EMPTY);
                } else if (c == '#') {
                    map.put(p, Status.OBSTACLE);
                } else {
                    map.put(p, Status.VISITED);
                }
            }
        }
        return map;
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
