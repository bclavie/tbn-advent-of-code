namespace _2024;

public class Day01 : Solution<(IEnumerable<int> Left, IEnumerable<int> Right), int>
{
	public override int DayNum => 1;

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