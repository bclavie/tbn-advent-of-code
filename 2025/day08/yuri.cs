var points = new HashSet<Point>(File.ReadLines("input.txt").Select(line =>
{
    var parts = line.Split(",").Select(s => int.Parse(s)).ToArray();
    return new Point(parts[0], parts[1], parts[2]);
}));

var distancesBetweenPoints = new Dictionary<(Point A, Point B), double>();

// every point is its own circuit to start with.
var circuits = points.Select(x => new HashSet<Point>() { x }).ToList();

foreach (var pointA in points)
{
    foreach (var pointB in points.Where(p => !p.Equals(pointA)))
    {
        // Just adding point combinations in order of X/Y/Z to avoid duplicates, probably not the best
        Point[] sorted = new Point[] { pointA, pointB }.OrderBy(x => x.X).ThenBy(x => x.Y).ThenBy(x => x.Z).ToArray();
        distancesBetweenPoints.TryAdd((sorted[0], sorted[1]), GetEuclideanDistance(pointA, pointB));
    }
}

var sortedByDistance = new Queue<(Point A, Point B, double Distance)>(distancesBetweenPoints
    .OrderBy(x => x.Value)
    .Select(x => (x.Key.A, x.Key.B, Distance: x.Value)));

for (int i = 0; i < 1000; i++)
{
    ConnectPoints(sortedByDistance.Dequeue(), circuits);
}

Console.WriteLine($"Part 1: {circuits.Select(x => x.Count).OrderByDescending(x => x).Take(3).Aggregate((a, b) => a * b)}");

(Point A, Point B, double Distance) conn = default;
while (circuits.Count > 1)
{
    conn = sortedByDistance.Dequeue();
    ConnectPoints(conn, circuits);
}

Console.WriteLine($"Part 2: {conn.A.X * conn.B.X}");

void ConnectPoints((Point A, Point B, double Distance) connection, List<HashSet<Point>> circuits)
{
    var circuitA = circuits.FirstOrDefault(x => x.Contains(connection.A));
    var circuitB = circuits.FirstOrDefault(x => x.Contains(connection.B));

    if (circuitA == circuitB)
    {
        // Already in the same circuit, do nothing
    }
    else if (circuitA is not null && circuitB is not null)
    {
        // Merge circuits        
        circuitA.UnionWith(circuitB);
        circuits.Remove(circuitB);
    }
    else if (circuitA is not null)
    {
        circuitA.Add(connection.B);
    }
    else if (circuitB is not null)
    {
        circuitB.Add(connection.A);
    }
}

double GetEuclideanDistance(Point a, Point b)
{
    long deltaX = a.X - b.X;
    long deltaY = a.Y - b.Y;
    long deltaZ = a.Z - b.Z;

    return Math.Sqrt((deltaX * deltaX) + (deltaY * deltaY) + (deltaZ * deltaZ));
}

struct Point
{
    public int X { get; init; }
    public int Y { get; init; }
    public int Z { get; init; }

    public Point(int x, int y, int z)
    {
        X = x;
        Y = y;
        Z = z;
    }

    public override string ToString() => $"({X},{Y},{Z})";
}