var beams = new HashSet<int>();
var splitCount = 0;

foreach (var line in File.ReadLines("input-example.txt"))
{
    if (beams.Count == 0)
    {
        beams.Add(line.IndexOf('S'));
        continue;
    }

    var nextBeams = new HashSet<int>();

    foreach (var x in beams)
    {
        if (line[x] == '^')
        {
            splitCount++;

            nextBeams.Add(x + 1);
            nextBeams.Add(x - 1);
        }
        else
        {
            nextBeams.Add(x);
        }
    }

    beams = nextBeams;
}

Console.WriteLine($"Part 1: {splitCount}");

var lines = File.ReadAllLines("input.txt").ToArray();
var timelineBeams = new long[lines[0].Length];
timelineBeams[lines[0].IndexOf('S')] = 1;

foreach (var line in lines.Skip(1))
{
    for (var x = 0; x < timelineBeams.Length; x++)
    {
        if (line[x] == '^' && timelineBeams[x] > 0)
        {
            timelineBeams[x + 1] += timelineBeams[x];
            timelineBeams[x - 1] += timelineBeams[x];
            timelineBeams[x] = 0;
        }
    }
}

Console.WriteLine($"Part 2: {timelineBeams.Sum()}");