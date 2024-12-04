using System.Text.RegularExpressions;

namespace _2024;

public class Day03 : Solution<string, int>
{
	public override int DayNum => 3;
	private static readonly Regex mulPattern = new Regex(@"mul\((\d+),(\d+)\)");

	public override string Parse(string[] lines)
	{
		return string.Concat(lines);
	}

	public override int Part1(string input)
	{
		var total = 0;

		foreach (var match in mulPattern.Matches(input).Cast<Match>())
		{
			total += int.Parse(match.Groups[1].Value) * int.Parse(match.Groups[2].Value);
		}

		return total;
	}

	public override int Part2(string input)
	{
		// Split input by do() so we know mul instructions are enabled at the start of each of these strings
		var doStrings = input.Split("do()");

		var total = 0;

		foreach (var str in doStrings)
		{
			var dontIdx = str.IndexOf("don't()");

			// If there's a don't() instruction in the string, only match up until that
			foreach (var match in mulPattern.Matches(dontIdx == -1 ? str : str.Substring(0, dontIdx)).Cast<Match>())
			{
				total += int.Parse(match.Groups[1].Value) * int.Parse(match.Groups[2].Value);
			}
		}
		return total;
	}
}