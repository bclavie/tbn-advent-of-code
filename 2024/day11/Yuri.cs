using _2024;

public class Day11 : Solution<List<ulong>, ulong>
{
    public override int DayNum => 11;
    private readonly Dictionary<(ulong, int), ulong> _resultCache = [];

    public override List<ulong> Parse(string[] lines)
    {
        return lines[0].Split(" ").Select(ulong.Parse).ToList();
    }

    ulong Blink(ulong stone, int numBlinksRemaining)
    {
        if (numBlinksRemaining == 0)
        {
            return 1;
        }

        numBlinksRemaining--;

        if (_resultCache.TryGetValue((stone, numBlinksRemaining), out var cachedNumStones))
        {
            return cachedNumStones;
        }

        if (stone == 0)
        {
            return Blink(1, numBlinksRemaining);
        }

        var str = stone.ToString();
        var mid = str.Length / 2;

        ulong[] newStones = str.Length % 2 == 0
            ? [ulong.Parse(str[..mid]), ulong.Parse(str[mid..])]
            : [stone * 2024];

        ulong numStones = newStones
            .Select(x => Blink(x, numBlinksRemaining))
            .Aggregate((a, b) => a + b);

        _resultCache.TryAdd((stone, numBlinksRemaining), numStones);
        return numStones;
    }

    public override ulong Part1(List<ulong> input)
    {
        return input
            .Select(x => Blink(x, 25))
            .Aggregate((a, b) => a + b);
    }

    public override ulong Part2(List<ulong> input)
    {
        return input
            .Select(x => Blink(x, 75))
            .Aggregate((a, b) => a + b);
    }
}