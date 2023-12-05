package tbn.aoc2023;

import java.io.File;
import java.io.FileNotFoundException;
import java.util.*;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Day5 {

    public static final Pattern numberPattern = Pattern.compile("\\d+");

    public static void main(String[] args) throws FileNotFoundException {
        part1();
        part2();
    }

    public static void part2() throws FileNotFoundException {
        List<String> lines = readFile("Day5.txt");
        lines.removeAll(List.of(""));
        List<Long> seeds = getSeeds(lines.get(0));
        List<Range> ranges = new ArrayList<>();
        Map<Integer, List<ConverterMap>> converters = getConverters(lines);
        for (int i = 0; i < seeds.size(); i += 2) {
            long min = seeds.get(i);
            long max = min + seeds.get(i + 1);

            ranges.add(new Range(min, max));
        }

        System.out.println(ranges);
        long min = Integer.MAX_VALUE;
        for (Range r : ranges) {
            System.out.println(r);
            for (long i = r.min; i < r.max; i++) {
                long convert = convert(i, converters);
                if (convert < min) {
                    min = convert;
                }
            }
        }
        System.out.println(min);
    }

    public static class Range {
        public long min;
        public long max;

        public Range(long min, long max) {
            this.min = min;
            this.max = max;
        }

        @Override
        public String toString() {
            return "Range{" +
                    "min=" + min +
                    ", max=" + max +
                    '}';
        }
    }

    public static void part1() throws FileNotFoundException {
        List<String> lines = readFile("Day5.txt");
        lines.removeAll(List.of(""));
        List<Long> seeds = getSeeds(lines.get(0));

        Map<Integer, List<ConverterMap>> converters = getConverters(lines);


        List<Long> soil = new ArrayList<>();
        for (Long seed : seeds) {
            soil.add(convert(seed, converters));
        }

        long min = Integer.MAX_VALUE;
        for (Long i : soil) {
            if (i < min) {
                min = i;
            }
        }

        System.out.println(min);
    }

    public static Map<Integer, List<ConverterMap>> getConverters(List<String> lines) {
        lines.removeAll(List.of(""));
        lines.remove(0);
        lines.remove(0);
        Map<Integer, List<ConverterMap>> converters = new HashMap<>();

        for (int i = 0; i < 7; i++) {
            converters.put(i, new ArrayList<>());
        }

        int index = 0;

        for (String s : lines) {
            ConverterMap converterMap = readConverterMapFromLine(s);
            if (converterMap != null) {
                converters.get(index).add(converterMap);
            } else {
                index++;
            }
        }


        return converters;
    }

    public static long convert(Long number, Map<Integer, List<ConverterMap>> converters) {
        for (int i = 0; i < converters.size(); i++) {
            number = convert(number, converters.get(i));
        }

        return number;
    }

    public static long convert(long number, List<ConverterMap> converters) {
        for (ConverterMap c : converters) {
            if (inRange(number, c)) {
                long convert = convert(number, c);
                //System.out.println(number + " in range " + c + " gives: " + convert);
                return convert;
            }
        }
        return number;
    }

    public static long convert(long number, ConverterMap converter) {
        return converter.targetStart + (number - converter.sourceStart);
    }

    public static boolean inRange(long number, ConverterMap map) {
        return number >= map.sourceStart && number < map.sourceStart + map.range;
    }

    public static ConverterMap readConverterMapFromLine(String line) {
        Matcher matcher = numberPattern.matcher(line);
        if (matcher.find()) {
            String destination = line.substring(matcher.start(), matcher.end());
            matcher.find(matcher.end());
            String source = line.substring(matcher.start(), matcher.end());
            matcher.find(matcher.end());
            String range = line.substring(matcher.start(), matcher.end());

            return new ConverterMap(source, destination, range);
        } else {
            return null;
        }
    }

    public static List<Long> getSeeds(String line) {
        Matcher matcher = numberPattern.matcher(line);
        List<Long> result = new ArrayList<>();
        int i = 0;
        while (matcher.find(i)) {
            i = matcher.end();
            result.add(Long.valueOf(line.substring(matcher.start(), matcher.end())));
        }

        return result;
    }

    public static class ConverterMap {
        public long sourceStart;

        public long targetStart;

        public long range;

        public ConverterMap(String source, String destination, String range) {
            this.sourceStart = Long.parseLong(source);
            this.targetStart = Long.parseLong(destination);
            this.range = Long.parseLong(range);
        }

        @Override
        public String toString() {
            return "ConverterMap{" +
                    "sourceStart=" + sourceStart +
                    ", targetStart=" + targetStart +
                    ", range=" + range +
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
