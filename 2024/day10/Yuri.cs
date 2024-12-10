using _2024;
using System.Drawing;

public class Day10 : Solution<int[][], int>
{
    public override int DayNum => 10;

    public override int[][] Parse(string[] lines)
    {
        return lines.Select(l => l.Select(c => int.Parse(c.ToString())).ToArray()).ToArray();
    }

    public static bool IsInBounds(int[][] input, Point point)
    {
        return
            point.Y >= 0 && point.Y < input.Length &&
            point.X >= 0 && point.X < input[0].Length;
    }

    public static Dictionary<Point, int> Score(int[][] input, Point pos, Dictionary<Point, int> scored, int? previousValue = -1)
    {
        var (y, x) = (pos.Y, pos.X);

        if (!IsInBounds(input, pos))
        {
            return [];
        }

        var currentValue = input[y][x];
        if (currentValue != previousValue + 1)
        {
            return [];
        }

        if (currentValue == 9)
        {
            if (scored.TryGetValue(pos, out var value))
            {
                scored[pos]++;
            }
            else
            {
                scored.Add(pos, 1);
            }
        }

        Point[] neighbours = [new Point(x - 1, y), new Point(x + 1, y), new Point(x, y - 1), new Point(x, y + 1)];
        foreach (var neighbour in neighbours)
        {
            Score(input, neighbour, scored, currentValue);
        }

        return scored;
    }

    public override int Part1(int[][] input)
    {
        var total = 0;

        for (int y = 0; y < input.Length; y++)
        {
            for (int x = 0; x < input[y].Length; x++)
            {
                if (input[y][x] == 0)
                {
                    total += Score(input, new Point(x, y), []).Count;
                }
            }
        }

        return total;
    }

    public override int Part2(int[][] input)
    {
        var total = 0;

        for (int y = 0; y < input.Length; y++)
        {
            for (int x = 0; x < input[y].Length; x++)
            {
                if (input[y][x] == 0)
                {
                    total += Score(input, new Point(x, y), []).Sum(x => x.Value);
                }
            }
        }

        return total;
    }
}