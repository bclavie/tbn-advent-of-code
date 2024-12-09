using _2024;

public class Day9 : Solution<int[], long>
{
    public override int DayNum => 9;

    public override int[] Parse(string[] lines)
    {
        var line = lines[0];
        var results = new List<int>();
        var id = 0;

        foreach (var (c, idx) in line.Select((c, idx) => (int.Parse(c.ToString()), idx)))
        {
            var isFreeSpace = idx % 2 != 0;
            results.AddRange(Enumerable.Range(0, c).Select(_ => isFreeSpace ? -1 : id));

            if (!isFreeSpace)
            {
                id++;
            }
        }

        return [.. results];
    }

    public override long Part1(int[] input)
    {
        // clone input array because we'll be swapping in place
        int[] arr = [.. input];
        var idxFirstFreeSpace = 0;

        for (int i = arr.Length - 1; i > idxFirstFreeSpace; i--)
        {
            var curr = arr[i];

            // skip if this element is free space
            if (curr == -1)
            {
                continue;
            }

            idxFirstFreeSpace = Array.FindIndex(arr, x => x == -1);
            if (idxFirstFreeSpace >= i)
            {
                break;
            }

            // swap with first free space in the array
            arr[idxFirstFreeSpace] = curr;
            arr[i] = -1;
        }

        return CalculateChecksum(arr);
    }

    class FileSystemBlock
    {
        public int IdxFrom { get; set; }
        public int IdxTo { get; set; }
        public int Width { get; set; }
        public int Value { get; set; }

        public FileSystemBlock(int idxFrom, int idxTo, int width, int value)
        {
            IdxFrom = idxFrom;
            IdxTo = idxTo;
            Width = width;
            Value = value;
        }
    }

    public override long Part2(int[] input)
    {
        var blocks = new List<FileSystemBlock>();
        int currentBlockStartIdx = 0;
        int currentId = input[0];

        foreach (var (x, idx) in input.Select((x, idx) => (x, idx)))
        {
            if (x != currentId)
            {
                blocks.Add(new FileSystemBlock(currentBlockStartIdx, idx - 1, idx - currentBlockStartIdx, currentId));
                currentBlockStartIdx = idx;
                currentId = x;
            }
        }
        blocks.Add(new FileSystemBlock(currentBlockStartIdx, input.Length - 1, input.Length - currentBlockStartIdx, currentId));

        for (int val = blocks.Max(x => x.Value); val > 0; val--)
        {
            var curr = blocks.FirstOrDefault(x => x.Value == val);
            if (curr is null)
            {
                continue;
            }

            var firstAvailableSpace = blocks.Where(x => x.Value == -1 && x.IdxTo < curr.IdxFrom && x.Width >= curr.Width).FirstOrDefault();
            if (firstAvailableSpace is null)
            {
                // no free space big enough available
                continue;
            }

            // If they're the same width, we can straight swap
            if (firstAvailableSpace.Width == curr.Width)
            {
                firstAvailableSpace.Value = val;

                curr.Value = -1;
                continue;
            }

            // If the space is bigger, we'll have to add in a new values element to represent the remaining free space
            if (firstAvailableSpace.Width > curr.Width)
            {
                var remainder = firstAvailableSpace.Width - curr.Width;
                firstAvailableSpace.Width = curr.Width;
                firstAvailableSpace.Value = curr.Value;
                firstAvailableSpace.IdxTo -= remainder;
                blocks.Insert(blocks.IndexOf(firstAvailableSpace) + 1, new FileSystemBlock(firstAvailableSpace.IdxTo + 1, firstAvailableSpace.IdxTo + remainder, remainder, -1));

                curr.Value = -1;
            }
        }

        // Reassemble array based on my stupid type and then run checksum calculation on it
        return CalculateChecksum(blocks.SelectMany(x => Enumerable.Repeat(x.Value, x.Width)).ToArray());
    }

    private static long CalculateChecksum(int[] filesystem) => filesystem.Select((x, idx) => (Value: (long)x, Index: (long)idx)).Sum(x => x.Value == -1L ? 0 : x.Value * x.Index);
}