using _2024;

public class Equation
{
    public ulong Outcome { get; init; }
    public ulong[] Values { get; init; }

    public Equation(ulong outcome, ulong[] values)
    {
        Outcome = outcome;
        Values = values;
    }

    public bool CanBeSolved(bool enableConcat)
    {
        return CanBeSolvedRec(Values[0], 1, enableConcat);
    }

    private bool CanBeSolvedRec(ulong current, int index, bool enableConcat)
    {
        // Exceeded outcome so this branch is always going to be false, we don't need to calculate further steps
        if (current > Outcome)
        {
            return false;
        }

        if (index >= Values.Length)
        {
            return current == Outcome;
        }

        // Addition
        if (CanBeSolvedRec(current + Values[index], index + 1, enableConcat))
            return true;

        // Multiplication
        if (CanBeSolvedRec(current * Values[index], index + 1, enableConcat))
            return true;

        // Concatenation
        if (enableConcat && CanBeSolvedRec(ulong.Parse($"{current}{Values[index]}"), index + 1, enableConcat))
        {
            return true;
        }

        return false;
    }

}

public class Day7 : Solution<Equation[], ulong>
{
    public override int DayNum => 7;

    public override Equation[] Parse(string[] lines)
    {
        return lines.Select(line =>
        {
            var parts = line.Split(':');
            ulong outcome = ulong.Parse(parts[0]);
            var values = parts[1].Trim().Split(' ').Select(ulong.Parse).ToArray();

            return new Equation(outcome, values);
        }).ToArray();
    }

    public override ulong Part1(Equation[] input)
    {
        ulong total = 0;
        foreach (var equation in input)
        {
            if (equation.CanBeSolved(false))
            {
                total += equation.Outcome;
            }
        }
        return total;
    }

    public override ulong Part2(Equation[] input)
    {
        ulong total = 0;
        foreach (var equation in input)
        {
            if (equation.CanBeSolved(true))
            {
                total += equation.Outcome;
            }
        }
        return total;
    }
}