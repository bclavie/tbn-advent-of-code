using _2024;
using System.Drawing;

public readonly struct Antenna
{
    public readonly Point Location { get; init; }
    public readonly char Character { get; init; }

    public Antenna(Point location, char character)
    {
        Location = location;
        Character = character;
    }
}

public class Day8 : Solution<(Antenna[] Antennas, int maxX, int maxY), int>
{
    public override int DayNum => 8;

    public override (Antenna[] Antennas, int maxX, int maxY) Parse(string[] lines)
    {
        var antennas = new List<Antenna>();

        for (int y = 0; y < lines.Length; y++)
        {
            var line = lines[y];
            for (int x = 0; x < line.Length; x++)
            {
                if (line[x] != '.')
                    antennas.Add(new Antenna(new Point(x, y), line[x]));
            }
        }

        return (antennas.ToArray(), lines.Max(x => x.Length), lines.Length);
    }

    Point[] GetAntiNodesP1((Point p1, Point p2) pointPair, int maxX, int maxY)
    {
        var p1 = pointPair.p1;
        var p2 = pointPair.p2;

        var diffX = p2.X - p1.X;
        var diffY = p2.Y - p1.Y;

        Point[] antiNodes = [new Point(p1.X - diffX, p1.Y - diffY),
            new Point(p2.X + diffX, p2.Y + diffY)];

        // return the anti nodes that were in bounds
        return antiNodes.Where(n =>
            n.X >= 0 && n.X < maxX &&
            n.Y >= 0 && n.Y < maxY).ToArray();
    }

    IEnumerable<(Point p1, Point p2)> GetAllPointPairs(Point[] points)
    {
        for (int i = 0; i < points.Length; i++)
        {
            for (int j = i + 1; j < points.Length; j++)
            {
                yield return (points[i], points[j]);
            }
        }
    }

    public override int Part1((Antenna[] Antennas, int maxX, int maxY) input)
    {
        var antiNodeLocations = new HashSet<Point>();
        var pointsByCharacter = input.Antennas
            .GroupBy(x => x.Character)
            .Select(group => group.Select(x => x.Location).ToArray());

        foreach (var antiNode in pointsByCharacter
            .SelectMany(x => GetAllPointPairs(x).SelectMany(x => GetAntiNodesP1(x, input.maxX, input.maxY))))
        {
            antiNodeLocations.Add(antiNode);
        }

        return antiNodeLocations.Count;
    }

    Point[] GetAntiNodesP2((Point p1, Point p2) pointPair, int maxX, int maxY)
    {
        bool PointIsWithinBounds(Point p) => p.X >= 0 && p.X < maxX &&
            p.Y >= 0 && p.Y < maxY;

        var p1 = pointPair.p1;
        var p2 = pointPair.p2;

        var diffX = p2.X - p1.X;
        var diffY = p2.Y - p1.Y;
        var antiNodes = new List<Point>() { p1, p2 };

        // add antinodes in one direction until out of bounds
        while (PointIsWithinBounds(p1))
        {
            p1 = new Point(p1.X - diffX, p1.Y - diffY);
            antiNodes.Add(p1);
        }

        // and the other direction
        while (PointIsWithinBounds(p2))
        {
            p2 = new Point(p2.X + diffX, p2.Y + diffY);
            antiNodes.Add(p2);
        }

        // return the anti nodes that were within bounds
        return antiNodes.Where(p =>
            p.X >= 0 && p.X < maxX &&
            p.Y >= 0 && p.Y < maxY).ToArray();
    }

    public override int Part2((Antenna[] Antennas, int maxX, int maxY) input)
    {
        var antiNodeLocations = new HashSet<Point>();
        var pointsByCharacter = input.Antennas
            .GroupBy(x => x.Character)
            .Select(group => group.Select(x => x.Location).ToArray());

        foreach (var antiNode in pointsByCharacter
            .SelectMany(x => GetAllPointPairs(x).SelectMany(x => GetAntiNodesP2(x, input.maxX, input.maxY))))
        {
            antiNodeLocations.Add(antiNode);
        }

        return antiNodeLocations.Count;
    }
}