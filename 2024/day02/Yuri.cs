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
		Trend? reportTrend = null;

		for (int i = 1; i < report.Length; i++)
		{
			var currentLevel = report[i];
			var previousLevel = report[i - 1];

			var difference = currentLevel - previousLevel;

			if (i == 1)
			{
				reportTrend = difference < 0 ? Trend.Decreasing : Trend.Increasing;
			}

			var levelIsSafe = !(difference == 0 ||
				reportTrend != (difference < 0 ? Trend.Decreasing : Trend.Increasing) ||
				Math.Abs(difference) > 3);

			if (levelIsSafe)
				continue;

			if (!dampener)
			{
				return false;
			}

			// At this point the report is unsafe but we have a damper:
			// Check if any subsequence of this report with 1 level removed would be safe
			for (int j = 0; j < report.Length; j++)
			{
				if (IsSafe(SkipLevel(report, j)))
				{
					return true;
				}
			}

			return false;
		}

		return true;
	}

	private static int[] SkipLevel(int[] report, int levelToSkip)
	{
		var damped = report.ToList();
		damped.RemoveAt(levelToSkip);
		return damped.ToArray();
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