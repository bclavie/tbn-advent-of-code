namespace _2024;

public class Day01v1 : Solution<(IEnumerable<int> Left, IEnumerable<int> Right), int>
{
	public override int DayNum => 1;
	public override string? Version => "v1 - LINQ";

	public override (IEnumerable<int> Left, IEnumerable<int> Right) Parse(string[] lines)
	{
		var pairs = lines.Select(line =>
		{
			var parts = line.Split("   ");
			return (Left: int.Parse(parts[0]), Right: int.Parse(parts[1]));
		}).ToList();

		return (
			Left: pairs.Select(x => x.Left),
			Right: pairs.Select(x => x.Right)
		);
	}

	public override int Part1((IEnumerable<int> Left, IEnumerable<int> Right) input) =>
		input.Left.OrderByDescending(x => x)
			.Zip(input.Right.OrderByDescending(x => x))
			.Sum(x => Math.Abs(x.First - x.Second));

	public override int Part2((IEnumerable<int> Left, IEnumerable<int> Right) input) =>
		input.Left.Sum(l => l * input.Right.Count(r => r == l));
}

public class Day01v2 : Solution<(int[] LeftSorted, int[] RightSorted), int>
{
	public override int DayNum => 1;

	public override string? Version => "v2 - Array.Sort + loops";

	public override (int[] LeftSorted, int[] RightSorted) Parse(string[] lines)
	{
		int[] left = new int[lines.Length];
		int[] right = new int[lines.Length];

		for (int i = 0; i < lines.Length; i++)
		{
			var parts = lines[i].Split("   ");
			left[i] = int.Parse(parts[0]);
			right[i] = int.Parse(parts[1]);
		}

		Array.Sort(left);
		Array.Sort(right);

		return (left, right);
	}

	public override int Part1((int[] LeftSorted, int[] RightSorted) input)
	{
		var totalDifference = 0;

		for (var i = 0; i < input.LeftSorted.Length; i++)
		{
			totalDifference += Math.Abs(input.LeftSorted[i] - input.RightSorted[i]);
		}

		return totalDifference;
	}

	public override int Part2((int[] LeftSorted, int[] RightSorted) input)
	{
		var total = 0;
		Dictionary<int, int> numberOccurrences = [];

		foreach (var r in input.RightSorted)
		{
			if (!numberOccurrences.TryGetValue(r, out int value))
			{
				numberOccurrences.Add(r, 1);
			}
			else
			{
				numberOccurrences[r] = ++value;
			}
		}

		for (var leftIdx = 0; leftIdx < input.LeftSorted.Length; leftIdx++)
		{
			var left = input.LeftSorted[leftIdx];
			numberOccurrences.TryGetValue(left, out int multiplier);
			total += left * multiplier;
		}

		return total;
	}
}