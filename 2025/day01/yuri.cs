using System.IO;

var instructions = File.ReadLines("input").Select(str => new Instruction(str[0], int.Parse(str[1..^0])));

// Part 1
var timesAtZero = 0;
var curr = 50;
foreach (var instruction in instructions)
{
    if (instruction.Direction == 'L')
    {
        curr -= instruction.Clicks;
    }
    else if (instruction.Direction == 'R')
    {
        curr += instruction.Clicks;
    }

    if (curr % 100 == 0)
    {
        timesAtZero++;
    }
}
Console.WriteLine($"Part 1: {timesAtZero}");

// Part 2
timesAtZero = 0;
curr = 50;
foreach (var instruction in instructions)
{
    var next = curr;
    if (instruction.Direction == 'L')
    {
        next -= instruction.Clicks;
    }
    else if (instruction.Direction == 'R')
    {
        next += instruction.Clicks;
    }

    timesAtZero += Math.Abs(next / 100 - curr / 100); // Times landed on 0, difference in hundredths between starting and next number

    // Account for crossing zero or landing on zero, could probably do this all in one go with better maths and avoiding relying on truncated division?
    if ((next > 0 && curr < 0) || (next < 0 && curr > 0) || next == 0) 
    {
        timesAtZero++;
    }

    //Console.WriteLine($"{instruction.Direction}{instruction.Clicks,4}:\tCurr: {curr,4}\tNext: {next,4}\t{result}");
    curr = next % 100;
}
Console.WriteLine($"Part 2: {timesAtZero}");

record Instruction(char Direction, int Clicks);