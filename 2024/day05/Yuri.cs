using _2024;

public class Day5 : Solution<(int[][] Rules, int[][] Updates), int>
{

	public override int DayNum => 5;

	public override (int[][] Rules, int[][] Updates) Parse(string[] lines)
	{
		var rules = new List<int[]>();
		var updates = new List<int[]>();

		foreach (var line in lines)
		{
			if (string.IsNullOrEmpty(line))
			{
				continue;
			}

			if (line.Contains('|'))
			{
				rules.Add(line.Split('|').Select(int.Parse).ToArray());
				continue;
			}

			updates.Add(line.Split(',').Select(int.Parse).ToArray());
		}

		return (rules.ToArray(), updates.ToArray());
	}

	private static bool IsUpdateViolatingRule(int[] rule, int[] update)
	{
		if (!update.Contains(rule[1]) || !update.Contains(rule[0]))
		{
			return false;
		}

		return Array.IndexOf(update, rule[0]) > Array.IndexOf(update, rule[1]);
	}

	public override int Part1((int[][] Rules, int[][] Updates) input)
	{
		return input.Updates
			.Where(update => !input.Rules.Any(rule => IsUpdateViolatingRule(rule, update)))
			.Sum(update => update[update.Length / 2]);
	}

	public override int Part2((int[][] Rules, int[][] Updates) input)
	{
		var total = 0;

		foreach (var update in input.Updates.Where(x => input.Rules.Any(rule => IsUpdateViolatingRule(rule, x))))
		{
			var applicableRules = input.Rules.Where(rule => update.Contains(rule[0]) && update.Contains(rule[1])).ToArray();

			var orderedPagesInRule = applicableRules.SelectMany(x => x.Select(y => y))
				.Distinct()
				.OrderByDescending(x => applicableRules.Count(y => y[0] == x)).ToArray();

			total += update
				.OrderBy(x => Array.IndexOf(orderedPagesInRule, x))
				.ElementAt(update.Length / 2);
		}

		return total;
	}
}