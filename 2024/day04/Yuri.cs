using System.Drawing;

namespace _2024;

public class Day4 : Solution<char[][], int>
{
	public override int DayNum => 4;

	public override char[][] Parse(string[] lines)
	{
		return lines.Select(x => x.ToArray()).ToArray();
	}

	private static char? TryGetLetter(char[][] input, Point point)
	{
		if (input == null)
			return null;

		if (point.X < 0 ||
			point.Y < 0 ||
			point.Y >= input.Length ||
			point.X >= input[point.Y].Length)
		{
			return null;
		}

		return input[point.Y][point.X];
	}

	private enum Direction
	{
		Left,
		Right,
		Up,
		Down,
		UpRight,
		UpLeft,
		DownRight,
		DownLeft
	}

	private static Point GetNextPointInDirection(Point point, Direction direction) =>
		direction switch
		{
			Direction.Left => new Point(point.X - 1, point.Y),
			Direction.Right => new Point(point.X + 1, point.Y),
			Direction.Up => new Point(point.X, point.Y - 1),
			Direction.Down => new Point(point.X, point.Y + 1),
			Direction.UpRight => new Point(point.X + 1, point.Y - 1),
			Direction.UpLeft => new Point(point.X - 1, point.Y - 1),
			Direction.DownRight => new Point(point.X + 1, point.Y + 1),
			Direction.DownLeft => new Point(point.X - 1, point.Y + 1),
			_ => throw new NotImplementedException()
		};

	private static Point[] WordsFromPoint(char[][] input, Point point, char[] word, Direction searchDirection)
	{
		// Out of bounds or didn't match the next character of the word, dead end
		var currentLetter = TryGetLetter(input, point);
		if (currentLetter is null || word[0] != currentLetter)
		{
			return [];
		}

		// We're the last character and the word has been found
		if (word.Length == 1 && word[0] == currentLetter)
		{
			return [point];
		}

		var matchingPositionsInDirection = WordsFromPoint(input, GetNextPointInDirection(point, searchDirection), word[1..], searchDirection);
		if (matchingPositionsInDirection.Length == 0)
		{
			return [];
		}

		return [point, .. matchingPositionsInDirection];
	}

	public override int Part1(char[][] input)
	{
		var word = "XMAS".ToArray();
		var total = 0;

		for (var y = 0; y < input.Length; y++)
		{
			for (var x = 0; x < input[y].Length; x++)
			{
				var point = new Point(x, y);
				total += Enum.GetValues<Direction>().Count(direction => WordsFromPoint(input, point, word, direction).Length == word.Length);
			}
		}

		return total;
	}

	public override int Part2(char[][] input)
	{
		var word = "MAS".ToArray();
		var foundWords = new List<Point[]>();
		Direction[] diagonalDirections = [Direction.UpLeft, Direction.UpRight, Direction.DownLeft, Direction.DownRight];

		for (var y = 0; y < input.Length; y++)
		{
			for (var x = 0; x < input[y].Length; x++)
			{
				var point = new Point(x, y);
				var wordsFromPoint = diagonalDirections.Select(direction => WordsFromPoint(input, point, word, direction));
				foundWords.AddRange(wordsFromPoint.Where(x => x.Length > 0));
			}
		}

		return foundWords
			.GroupBy(word => word[1])
			.Where(group => group.Count() > 1)
			.Count();
	}
}