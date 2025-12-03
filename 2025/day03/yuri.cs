var batteryBanks = File.ReadAllLines("input.txt").Select(line => line.Select(x => x - '0').ToArray());

var part1 = batteryBanks.Sum(Part1);
Console.WriteLine($"Part 1: {part1}");

var part2 = batteryBanks.Sum(x => Part2(x, 12));
Console.WriteLine($"Part 2: {part2}");

int Part1(int[] batteryBank)
{
    var highestLeft = 0;
    var highestRight = 0;

    for (var i = 0; i < batteryBank.Length; i++)
    {
        if (batteryBank[i] > highestLeft && i != batteryBank.Length - 1)
        {
            highestLeft = batteryBank[i];
            highestRight = 0;
        }
        else if (batteryBank[i] > highestRight)
        {
            highestRight = batteryBank[i];
        }
    }

    return highestLeft * 10 + highestRight;
}

long Part2(int[] batteryBank, int numberOfBatteries)
{
    var highestJoltages = new int[numberOfBatteries];

    for (var batteryIdx = 0; batteryIdx < batteryBank.Length; batteryIdx++)
    {
        for (var highestJoltagesIdx = 0; highestJoltagesIdx < highestJoltages.Length; highestJoltagesIdx++)
        {
            // Not enough batteries remaining in bank to start a new sequence          
            if (highestJoltages.Length - highestJoltagesIdx > batteryBank.Length - batteryIdx)
            {
                continue;
            }

            if (batteryBank[batteryIdx] > highestJoltages[highestJoltagesIdx])
            {
                highestJoltages[highestJoltagesIdx] = batteryBank[batteryIdx];

                // Started new sequence so reset highest joltages from here onwards to 0
                Array.Clear(highestJoltages, highestJoltagesIdx + 1, highestJoltages.Length - (highestJoltagesIdx + 1));

                break;
            }
        }
    }

    var sum = 0L;

    for (int i = 0; i < highestJoltages.Length; i++)
    {
        sum = sum * 10 + highestJoltages[i];
    }

    return sum;
}