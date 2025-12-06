var lines = File.ReadLines("input.txt");

Func<string, long[]> parseLinePart1 = str => str.Split(' ', StringSplitOptions.RemoveEmptyEntries)
    .Select(x => long.Parse(x))
    .ToArray();

var operatorFunctions = new Dictionary<char, Func<long, long, long>>()
{
    ['*'] = (a, b) => a * b,
    ['+'] = (a, b) => a + b
};

var ops = lines.Last().Where(x => x != ' ').ToArray();
var problemTotals = parseLinePart1(lines.First());

foreach (var line in lines.Skip(1).Take(lines.Count() - 2))
{
    var parts = parseLinePart1(line);
    for (int i = 0; i < parts.Length; i++)
    {
        problemTotals[i] = operatorFunctions[ops[i]](problemTotals[i], parts[i]);
    }
}

Console.WriteLine($"Part 1: {problemTotals.Sum()}");

var opsStack = new Stack<char>(ops);
var opsIndices = lines.Last()
         .Select((ch, idx) => (ch, idx))
         .Where(t => t.ch != ' ')
         .Select(t => t.idx)
         .ToList();

var grandTotal = 0L;
var columnNumbers = new List<long>();

for (var characterIdx = lines.Max(x => x.Length) - 1; characterIdx >= 0; characterIdx--)
{
    var baseMult = 1;
    var charTotal = 0;

    for (var rowIdx = lines.Count() - 2; rowIdx >= 0; rowIdx--)
    {
        var c = lines.ElementAt(rowIdx).ElementAt(characterIdx);

        if (c != ' ')
        {
            charTotal += baseMult * (c - '0');
            baseMult *= 10;
        }

        if (rowIdx == 0)
        {
            columnNumbers.Add(charTotal);
        }
    }

    // Finished this column
    if (opsIndices.Contains(characterIdx))
    {
        grandTotal += columnNumbers.Aggregate(operatorFunctions[opsStack.Pop()]);
        columnNumbers = [];

        // Skip over the gap to the next column
        characterIdx--;
    }
}

Console.WriteLine($"Part 2: {grandTotal}");