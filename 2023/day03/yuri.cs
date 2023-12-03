using System.Drawing;

namespace tbn_advent_of_code_csharp._2023.day03
{
    internal class Day03
    {
        internal Day03()
        {
            var lines = File.ReadAllLines("input/day03.txt");

            Console.WriteLine($"Part 1: {Part1(lines)}");
            Console.WriteLine($"Part 2: {Part2(lines)}");
        }

        private List<PartNumber> GetPotentialPartNumbers(string[] lines)
        {
            var potentialPartNumbers = new List<PartNumber>();

            for (var i = 0; i < lines.Length; i++)
            {
                int partNumberStartIndex = -1;
                var partNumber = string.Empty;

                for (var j = 0; j < lines[i].Length; j++)
                {
                    var c = lines[i][j];

                    if (char.IsDigit(c))
                    {
                        if (partNumberStartIndex == -1)
                            partNumberStartIndex = j;

                        partNumber += c;
                    }

                    // Part number we were appending to has come to an end 
                    if (!string.IsNullOrEmpty(partNumber) && (!char.IsDigit(c) || j == lines[i].Length - 1))
                    {
                        potentialPartNumbers.Add(new PartNumber(partNumber, partNumberStartIndex, i));
                        
                        // Don't need to do this when reached end of line but w/e
                        partNumber = string.Empty;
                        partNumberStartIndex = -1;
                    }
                }
            }

            return potentialPartNumbers;
        }

        private int Part1(string[] lines)
        {
            var symbolCharacters = lines.SelectMany(line => line.Where(c => !char.IsDigit(c) && c != '.')).Distinct().ToList();
            var symbols = GetSymbolPoints(lines, symbolCharacters);
            var partNumbers = GetPotentialPartNumbers(lines);

            return symbols
                .SelectMany(x => GetAdjacentPartNumbers(x, partNumbers))
                .Distinct()
                .Sum(x => x.Number);
        }

        private int Part2(string[] lines)
        {
            var gearSymbols = GetSymbolPoints(lines, new List<char> { '*' });
            var partNumbers = GetPotentialPartNumbers(lines);

            return gearSymbols.Select(gearSymbol =>
            {
                var adjacentPartNumbers = GetAdjacentPartNumbers(gearSymbol, partNumbers);
                if (adjacentPartNumbers.Count != 2)
                    return 0;

                return adjacentPartNumbers.Select(x => x.Number).Aggregate((a, b) => a * b);
            }).Sum();
        }

        private List<Point> GetSymbolPoints(string[] lines, List<char> symbolCharacters)
        {
            return lines.SelectMany((line, lineIndex) =>
                line.Select((c, characterIndex) => Tuple.Create(c, characterIndex))
                    .Where(x => symbolCharacters.Contains(x.Item1))
                    .Select(x => new Point(x.Item2, lineIndex))).ToList();
        }

        private List<PartNumber> GetAdjacentPartNumbers(Point symbol, List<PartNumber> partNumbers)
        {
            var xRange = Enumerable.Range(symbol.X - 1, 3);
            var yRange = Enumerable.Range(symbol.Y - 1, 3);

            var adjacentPoints = yRange.SelectMany(y => xRange.Select(x => new Point(x, y)));

            return partNumbers
                .Where(partNumber =>
                    Enumerable.Range(partNumber.StartIndex, partNumber.String.Length)
                        .Select(x => new Point(x, partNumber.LineIndex))
                        .Intersect(adjacentPoints).Any())
                .ToList();
        }

        private class PartNumber
        {
            internal int Number { get; init; }
            internal string String { get; init; }
            internal int StartIndex { get; init; }
            internal int LineIndex { get; init; }

            public PartNumber(string partNumberStr, int startIndex, int line)
            {
                if (startIndex == -1)
                {
                    throw new ArgumentException(nameof(startIndex));
                }

                Number = int.Parse(partNumberStr);
                String = partNumberStr;
                StartIndex = startIndex;
                LineIndex = line;
            }
        }
    }
}
