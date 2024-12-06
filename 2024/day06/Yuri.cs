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

	readonly struct WalkStep : IEquatable<WalkStep>
	{
		public readonly Point Position;
		public readonly Direction Direction;

		public WalkStep(Point position, Direction direction)
		{
			Position = position;
			Direction = direction;
		}

		public bool Equals(WalkStep other) =>
			Position.X == other.Position.X &&
			Position.Y == other.Position.Y &&
			Direction == other.Direction;

		public override int GetHashCode() =>
			HashCode.Combine(Position.X, Position.Y, Direction);
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
		var visitedPositions = SimulateCompleteWalk(input, out _).Select(x => x.Position).Distinct().ToArray();

		int batchSize = Math.Max(1, visitedPositions.Length / Environment.ProcessorCount);
		var results = new int[Environment.ProcessorCount];

		Parallel.For(0, Environment.ProcessorCount, processorIndex =>
		{
			int start = processorIndex * batchSize;
			int end = processorIndex == Environment.ProcessorCount - 1 ? visitedPositions.Length : (processorIndex + 1) * batchSize;

			for (int i = start; i < end; i++)
			{
				var position = visitedPositions[i];
				if (position != guardStartingPos)
				{
					SimulateCompleteWalk(input, out bool didLoop, position);
					if (didLoop)
						results[processorIndex]++;
				}
			}
		});

		return results.Sum();
	}
}