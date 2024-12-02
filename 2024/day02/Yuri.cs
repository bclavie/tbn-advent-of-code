namespace _2024;

public class Day02 : Solution<int[][], int>
{
	public override int DayNum => 2;

	public override int[][] Parse(string[] lines)
	{
		var result = new int[lines.Length][];

		foreach (var (line, idx) in lines.Select((l, i) => (l, i)))
		{
			result[idx] = line.Split(' ').Select(int.Parse).ToArray();
		}

		return result;
	}

	private enum Trend
	{
		Decreasing,
		Increasing
	}

	private static bool IsSafe(int[] report, bool dampener = false)
	{
		bool badLevelEncountered = false;
		Trend? reportTrend = null;

		for (int i = 1; i < report.Length; i++)
		{
			var currentLevel = report[i];
			var previousLevel = report[i - 1];

			if (i == 1)
			{
				reportTrend = currentLevel - previousLevel < 0 ? Trend.Decreasing : Trend.Increasing;
			}

			var difference = currentLevel - previousLevel;

			if (difference == 0 ||
				reportTrend != (difference < 0 ? Trend.Decreasing : Trend.Increasing) ||
				Math.Abs(difference) > 3)
			{
				if (!dampener || badLevelEncountered)
					return false;

				badLevelEncountered = true;
			}
		}

		return true;
	}

	public override int Part1(int[][] input)
	{
		return input.Count(x => IsSafe(x));
	}

	public override int Part2(int[][] input)
	{
		return input.Count(x => IsSafe(x, true));
	}
}