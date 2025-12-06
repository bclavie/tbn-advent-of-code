using System.Drawing;

// Part 1
var lines = File.ReadLines("input.txt");
var gridSizeX = lines.Max(line => line.Length);
var gridSizeY = lines.Count();
var grid = new Dictionary<Point, int>();
var paperRolls = new HashSet<Point>();

foreach (var (line, y) in lines.Select((line, y) => (line, y)))
{
    for (var x = 0; x < line.Length; x++)
    {
        if (line[x] != '@')
        {
            continue;
        }

        var roll = new Point(x, y);
        paperRolls.Add(roll);
        foreach (var neighbour in GetNeighbours(roll, gridSizeX, gridSizeY))
        {
            grid[neighbour] = grid.GetValueOrDefault(neighbour) + 1;
        }
    }
}

var accessibleRolls = new HashSet<Point>(paperRolls.Where(paperRollPoint =>
    !grid.TryGetValue(paperRollPoint, out var numPaperRollNeighbours) || numPaperRollNeighbours < 4));
Console.WriteLine($"Part 1 {accessibleRolls.Count()}");

// Part 2
var removedRolls = new HashSet<Point>();

while (accessibleRolls.Any())
{
    var rollsToRemove = accessibleRolls.ToList();
    accessibleRolls.Clear();

    foreach (var toRemove in rollsToRemove)
    {
        if (!paperRolls.Remove(toRemove))
            continue;

        removedRolls.Add(toRemove);

        // Update neighbours of the now removed roll
        foreach (var neighbour in GetNeighbours(toRemove, gridSizeX, gridSizeY))
        {
            if (grid.TryGetValue(neighbour, out var numPaperRollNeighbours))
            {
                numPaperRollNeighbours--;
                grid[neighbour] = numPaperRollNeighbours;

                if (numPaperRollNeighbours < 4 && paperRolls.Contains(neighbour))
                {
                    // Neighbouring roll is now accessible
                    accessibleRolls.Add(neighbour);
                }
            }
        }
    }
}

Console.WriteLine($"Part 2: {removedRolls.Count()}");

IEnumerable<Point> GetNeighbours(Point point, int gridSizeX, int gridSizeY)
{
    var xMin = Math.Max(point.X - 1, 0);
    var xMax = Math.Min(point.X + 1, gridSizeX - 1);
    var yMin = Math.Max(point.Y - 1, 0);
    var yMax = Math.Min(point.Y + 1, gridSizeY - 1);

    for (var y = yMin; y <= yMax; y++)
    {
        for (var x = xMin; x <= xMax; x++)
        {
            if (x == point.X && y == point.Y)
            {
                continue;
            }

            yield return new Point(x, y);
        }
    }
}