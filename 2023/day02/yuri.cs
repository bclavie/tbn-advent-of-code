using System.Text.RegularExpressions;

namespace tbn_advent_of_code_2023.day02
{
    internal class yuri
    {
        static void Main(string[] args)
        {
            new FuckThisShit().aaaaaaaaaa();
        }
    }

    internal class FuckThisShit
    {
        internal void aaaaaaaaaa()
        {
            var lines = File.ReadAllLines("input/day02.txt");
            var games = lines.Select(ParseLine).ToList();

            var powers = new List<int>();
            foreach (var game in games)
            {
                var highestRed = game.Where(x => x.Colour == "red").Max(x => x.Count);
                var highestGreen = game.Where(x => x.Colour == "green").Max(x => x.Count);
                var highestBlue = game.Where(x => x.Colour == "blue").Max(x => x.Count);

                powers.Add(Math.Max(1, highestRed) * Math.Max(1, highestGreen) * Math.Max(1, highestBlue));
            }

            Console.WriteLine(powers.Sum());
        }

        List<CubeCount> ParseLine(string line)
        {
            var regex = new Regex(@"\d+ [a-z]*");

            return regex.Matches(line)
                .Select(x => x.Value.Split(" "))
                .Select(x => new CubeCount() { Colour = x[1], Count = int.Parse(x[0]) })
                .ToList();
        }        
    }
    internal record CubeCount 
    { 
        internal string Colour { get; init; }
        internal int Count { get; init; } 
    }
}
