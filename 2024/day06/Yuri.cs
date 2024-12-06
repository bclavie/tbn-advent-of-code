using System.Drawing;
using _2024;

public class Day6 : Solution<char[][], int>
{
	public override int DayNum => 6;

	public override char[][] Parse(string[] lines)
	{
		return lines.Select(line => line.ToArray()).ToArray();
	}

	private static Point FindGuardStartingPosition(char[][] input)
	{
		for (int y = 0; y < input.Length; y++)
		{
			for (int x = 0; x < input[y].Length; x++)
			{
				if (input[y][x] == '^')
					return new Point(x, y);
			}
		}

		throw new ArgumentException("Guard not found");
	}

	enum Direction
	{
		Left, Right, Up, Down
	}

	static bool IsWalkFinished(char[][] input, Point guardPosition, Direction direction)
	{
		return direction switch
		{
			Direction.Left when guardPosition.X == -1 => true,
			Direction.Right when guardPosition.X == input[guardPosition.Y].Length => true,
			Direction.Down when guardPosition.Y == input.Length => true,
			Direction.Up when guardPosition.Y == -1 => true,
			_ => false,
		};
	}

	struct WalkStep
	{
		public Point Position { get; init; }
		public Direction Direction { get; init; }

		public WalkStep(Point position, Direction direction)
		{
			Position = position;
			Direction = direction;
		}
	}


	static HashSet<WalkStep> SimulateCompleteWalk(char[][] input, out bool didLoop, Point? obstaclePos = null)
	{
		didLoop = false;
		var visitedSteps = new HashSet<WalkStep>();

		var guardPosition = FindGuardStartingPosition(input);
		var direction = Direction.Up;

		while (true)
		{
			// We've already been at this location walking in the same direction - so we're in a loop
			var step = new WalkStep(guardPosition, direction);
			if (visitedSteps.Contains(step))
			{
				didLoop = true;
				return visitedSteps;
			}

			visitedSteps.Add(step);

			// walk 
			var nextPosition = direction switch
			{
				Direction.Left => new Point(guardPosition.X - 1, guardPosition.Y),
				Direction.Right => new Point(guardPosition.X + 1, guardPosition.Y),
				Direction.Up => new Point(guardPosition.X, guardPosition.Y - 1),
				Direction.Down => new Point(guardPosition.X, guardPosition.Y + 1),
				_ => throw new NotImplementedException()
			};

			if (IsWalkFinished(input, nextPosition, direction))
			{
				break;
			}

			// If guard is walking into an obstacle
			if (nextPosition == obstaclePos || input[nextPosition.Y][nextPosition.X] == '#')
			{
				// Rotate clockwise 90 deg
				direction = direction switch
				{
					Direction.Left => Direction.Up,
					Direction.Up => Direction.Right,
					Direction.Right => Direction.Down,
					Direction.Down => Direction.Left,
					_ => throw new NotImplementedException()
				};
				continue;
			}

			guardPosition = nextPosition;
		}

		return visitedSteps;
	}

	public override int Part1(char[][] input)
	{
		bool didLoop = false;
		var visited = SimulateCompleteWalk(input, out didLoop);
		return new HashSet<Point>(visited.Select(x => x.Position)).Count; // unique points in 2d space visited
	}

	public override int Part2(char[][] input)
	{
		var guardStartingPos = FindGuardStartingPosition(input);
		bool didLoop = false;
		var visitedPoints = new HashSet<Point>(SimulateCompleteWalk(input, out didLoop).Select(x => x.Position));

		// All visited points where turning 90 degrees would put us on a path towards another obstacle
		var numberOfPotentialLoops = 0;
		foreach (var point in visitedPoints)
		{
			// Can't place obstacle on starting position
			if (point == guardStartingPos)
			{
				continue;
			}

			// Place obstacle here and see if it loops
			SimulateCompleteWalk(input, out didLoop, point);
			if (didLoop)
			{
				numberOfPotentialLoops++;
			}
		}

		return numberOfPotentialLoops;
	}
}