using _2024;
using System.Drawing;

public class Region
{
    public int Area => Plots.Count();
    public int TotalPerimeter => Plots.Sum(x => x.Perimeter);
    public required ICollection<Plot> Plots { get; init; }
}

public readonly struct Plot
{
    public Plot(Point point, int perimeter)
    {
        Point = point;
        Perimeter = perimeter;
    }

    public readonly Point Point { get; init; }
    public readonly int Perimeter { get; init; }
}

public class Day12 : Solution<List<Region>, int>
{
    public override int DayNum => 12;

    static Point[] GetNeighboursFromPoint(Point point)
    {
        return new Point[] {
            new(point.X - 1, point.Y),
            new(point.X + 1, point.Y),
            new(point.X, point.Y - 1),
            new(point.X, point.Y + 1)
        };
    }

    static HashSet<Plot> GetRegionPlots(Point point, char regionPlant, string[] lines, HashSet<Plot> plotsInRegion)
    {
        var plant = lines[point.Y][point.X];

        // If this is a different plant or already added to the region we skip
        if (plant != regionPlant || plotsInRegion.Any(x => x.Point == point))
        {
            return [];
        }

        var neighbours = GetNeighboursFromPoint(point)
            .Where(p => (p.Y >= 0 && p.Y < lines.Length) && (p.X >= 0 && p.X < lines[p.Y].Length) // is in bounds
        );

        // perimeter = the amount of this plot's neighbours that are in the same region subtracted from the maximum number of sides (4)
        var perimeter = 4 - neighbours.Count(n => lines[n.Y][n.X] == regionPlant);

        // add this plot to the current region
        plotsInRegion.Add(new Plot(point, perimeter));

        // do the same for neighbours of the same plant
        foreach (var neighbourPoint in neighbours
            .Where(n => lines[n.Y][n.X] == regionPlant && !plotsInRegion.Any(x => x.Point == n)))
        {
            GetRegionPlots(neighbourPoint, regionPlant, lines, plotsInRegion);
        }

        return plotsInRegion;
    }

    public override List<Region> Parse(string[] lines)
    {
        var visitedPoints = new HashSet<Point>();
        var regions = new List<Region>();

        for (int y = 0; y < lines.Length; y++)
        {
            for (int x = 0; x < lines[y].Length; x++)
            {
                var currentPoint = new Point(x, y);
                var plant = lines[y][x];
                HashSet<Plot> currentRegionPlots = [];

                if (visitedPoints.Contains(currentPoint))
                {
                    continue;
                }

                var plots = GetRegionPlots(currentPoint, plant, lines, currentRegionPlots);
                var region = new Region() { Plots = plots };
                regions.Add(region);
                visitedPoints.UnionWith(plots.Select(x => x.Point));
            }
        }

        return regions;
    }

    public override int Part1(List<Region> input)
    {
        return input.Sum(x => x.Area * x.TotalPerimeter);
    }

    int CountSidesInRegion(Region region)
    {
        var regionPoints = region.Plots.Select(x => x.Point).ToHashSet();
        var visited = new HashSet<string>();
        var totalSides = 0;

        foreach (var startPoint in regionPoints)
        {
            var neighbours = GetNeighboursFromPoint(startPoint);
            foreach (var neighbour in neighbours)
            {
                if (regionPoints.Contains(neighbour))
                    continue;

                var dx = neighbour.X - startPoint.X;
                var dy = neighbour.Y - startPoint.Y;

                // Visited 'fence panels' are V_Start_End or H_Start_End so sides aren't counted twice
                var key = dx != 0 ? $"V_{Math.Min(startPoint.X, startPoint.X + dx)}_{startPoint.Y}"
                                     : $"H_{startPoint.X}_{Math.Min(startPoint.Y, startPoint.Y + dy)}";

                if (visited.Contains(key))
                    continue;

                totalSides++;
                visited.Add(key);

                bool TraverseEdge(Point pos, int perpDx, int perpDy)
                {
                    var nextPos = new Point(pos.X + perpDx, pos.Y + perpDy);
                    var nextNeighbor = new Point(nextPos.X + dx, nextPos.Y + dy);

                    // Next position is not part of our region, or its neighbour is. Either way that's the end of the side in this direction
                    if (!regionPoints.Contains(nextPos) || regionPoints.Contains(nextNeighbor))
                        return false;


                    visited.Add(dx != 0 ? $"V_{Math.Min(nextPos.X, nextPos.X + dx)}_{nextPos.Y}"
                                       : $"H_{nextPos.X}_{Math.Min(nextPos.Y, nextPos.Y + dy)}");
                    return true;
                }

                // traverse edge in one direction
                var current = startPoint;
                while (TraverseEdge(current, -dy, dx))
                {
                    current = new Point(current.X - dy, current.Y + dx);
                }

                // and then the opposite direction
                current = startPoint;
                while (TraverseEdge(current, dy, -dx))
                {
                    current = new Point(current.X + dy, current.Y - dx);
                }
            }
        }

        return totalSides;
    }

    public override int Part2(List<Region> input)
    {
        return input.Sum(region => CountSidesInRegion(region) * region.Area);
    }
}