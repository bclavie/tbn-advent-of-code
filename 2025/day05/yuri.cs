var freshIngredientsRanges = new HashSet<IngredientIdRange>();
var freshCount = 0;

foreach (var line in File.ReadLines("input.txt"))
{
    if (line == string.Empty)
    {
        continue;
    }

    var parts = line.Split('-');

    if (parts.Length > 1)
    {
        var start = long.Parse(parts[0]);
        var end = long.Parse(parts[1]);

        freshIngredientsRanges.Add(new IngredientIdRange(start, end));
    }
    else
    {
        var id = long.Parse(parts[0]);

        if (freshIngredientsRanges.Any(x => x.Start <= id && x.End >= id))
        {
            freshCount++;
        }
    }
}

Console.WriteLine($"Part 1: {freshCount}");

// Merge overlapping ranges
var mergedRanges = new List<IngredientIdRange>();
var sortedRanges = freshIngredientsRanges.OrderBy(r => r.Start).ToList();

var current = sortedRanges[0];
for (var i = 0; i < sortedRanges.Count; i++)
{
    if (i == sortedRanges.Count - 1)
    {
        mergedRanges.Add(current);
        continue;
    }

    var next = sortedRanges[i + 1];

    if (next.Start <= current.End + 1)
    {
        var newEnd = Math.Max(next.End, current.End);
        current = current with { End = newEnd };
    }
    else
    {
        mergedRanges.Add(current);
        current = next;
    }
}

Console.WriteLine($"Part 2: {mergedRanges.Sum(x => x.End - x.Start + 1)}");

record IngredientIdRange(long Start, long End);