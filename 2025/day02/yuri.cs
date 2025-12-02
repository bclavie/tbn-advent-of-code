// Part 1
var ranges = File.ReadAllText("input.txt").Split(',').Select(ParseRange).ToList();
var sumInvalidIds = 0L;
foreach (var range in ranges)
{
    for (var i = range.Start; i <= range.End; i++)
    {
        if (IsInvalidId_Part1(i))
        {
            sumInvalidIds += i;
        }
    }
}

Console.WriteLine($"Part 1: {sumInvalidIds}");

// Part 2
sumInvalidIds = 0;
foreach (var range in ranges)
{
    for (var i = range.Start; i <= range.End; i++)
    {
        var digits = getDigits(i);
        if (Enumerable.Range(2, digits.Length).Any(numRepeats => IsInvalidId_Part2(digits, numRepeats)))
        {
            sumInvalidIds += i;
        }
    }
}

Console.WriteLine($"Part 2: {sumInvalidIds}");

bool IsInvalidId_Part1(long number)
{
    var digits = getDigits(number);
    if (digits.Length % 2 == 1)
    {
        return false;
    }

    for (var i = 0; i < digits.Length / 2; i++)
    {
        if (digits[i] != digits[digits.Length / 2 + i])
        {
            return false;
        }
    }

    return true;
}

bool IsInvalidId_Part2(long[] digits, int numRepeats)
{
    if (digits.Length % numRepeats != 0)
    {
        return false;
    }

    for (var i = 0; i < digits.Length / numRepeats; i++)
    {
        for (var j = i + digits.Length / numRepeats;
                j < digits.Length;
                j = j + digits.Length / numRepeats)
        {
            if (digits[i] != digits[j])
            {
                return false;
            }

            if (j == digits.Length - 1)
            {
                return true;
            }
        }
    }

    return false;
}

long[] getDigits(long number)
{
    if (number == 0)
    {
        return [0];
    }

    var tmp = number;
    var numberOfDigits = 0;

    while (tmp > 0)
    {
        tmp = tmp / 10;
        numberOfDigits++;
    }

    var result = new long[numberOfDigits];
    foreach (var digitIdx in Enumerable.Range(0, numberOfDigits).Reverse())
    {
        result[digitIdx] = number % 10;
        number = number / 10;
    }

    return result;
}

ProductIdRange ParseRange(string range)
{
    var span = range.AsSpan();
    var hyphenIdx = span.IndexOf('-');

    var start = long.Parse(span[0..hyphenIdx]);
    var end = long.Parse(span[(hyphenIdx + 1)..]);

    return new ProductIdRange(start, end);
}

record ProductIdRange(long Start, long End);