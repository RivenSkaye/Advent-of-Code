using System;
using System.Collections.Generic;
using System.IO;
using System.Text;

// Define the input file; test for debug, real deal for release
#if DEBUG
string infile = "../../../test_inputs/05.txt";
#else
string infile = "../../../inputs/05.txt";
#endif

// Read the AOC input
using StreamReader r = File.OpenText(infile);
char[] read = new char[r.BaseStream.Length];
r.Read(read, 0, (int)r.BaseStream.Length);

// Allocate some data space
List<Stack<char>>? stacks = null;
Stack<(int, int, int)>? moves = null;

// These side effects are entirely intentional
Stack<(int, int, int)> ParseInput(char[] input)
{
    int start_moves = 0; // This is where the move instructions start
    bool wrapped = false; // check if we have all our stacks initialized
    int stacknum = 0; // The current stack we're gonna add to
    int linewidth = 1; // Line width
    List<List<char>> inputstacks = new();
    stacks = new();
    for (int i = 0; i < input.Length; i++)
    {
        char cur = read[i];
        // This means we've processed all stacks
        if (char.IsNumber(cur))
        {
            start_moves = i + linewidth;
            for (int st = 0; st < inputstacks.Count; st++) stacks.Add(new(inputstacks[st]));
            break;
        }
        if (cur == '\n')
        {
            if (!wrapped)
            {
                linewidth = i;
                wrapped = true;
            }
            // Skip the first char of the next line, it's either:
            // whitespace for stacknumbers, so we break on the next iteration
            // or it's a [ to denote a stack item.
            // Skipping it is faster either way
            stacknum = 0;
            i += 1;
            continue;
        }
        List<char> curstack;
        bool skip = char.IsWhiteSpace(cur) || cur == '[' || cur == ']';
        if (!wrapped && (i % 4) == 1)
        {
            curstack = new();
            inputstacks.Add(curstack);
        }
        else if (skip)
        {
            if ((i % 4) == 1) stacknum += 1;
            continue;
        }
        else curstack = inputstacks[stacknum];
        stacknum += 1;
        if (!skip) curstack.Insert(0, cur);
    }
    int amnt = 0;
    int from = 0;
    int to = 0;
    StringBuilder num = new(3);
    List<(int, int, int)> movebuilder = new();
    for (int i = start_moves; i <= read.Length; i++)
    {
        // If all three are set, add the move and reset to 0
        if (amnt > 0 && from > 0 && to > 0)
        {
            movebuilder.Insert(0, (amnt, from, to));
            amnt = from = to = 0;
            if (i == read.Length) break;
        }
        // Get the next char
        char cur = read[i];
        // If the stringbuilder has contents and the next entry isn't a number,
        // set the next number and clear it
        if (!char.IsNumber(cur))
        {
            if (num.Length > 0)
            {
                // This conversion should never fail
                if (amnt == 0 && int.TryParse(num.ToString(), out amnt)) num.Clear();
                else if (from == 0 && int.TryParse(num.ToString(), out from)) num.Clear();
                else if (to == 0 && int.TryParse(num.ToString(), out to)) num.Clear();
                else Console.WriteLine($"Failed to parse {num}!");
            }
            continue;
        }
        num.Append(cur);
    }
    return new(movebuilder);
}

void PartOne()
{
    Console.WriteLine("Part one:");
    foreach ((int, int, int) move in moves)
    {
        int amnt = move.Item1;
        int from = move.Item2 - 1;
        int to = move.Item3 - 1;
        while (amnt > 0)
        {
            stacks![to].Push(stacks[from].Pop());
            amnt--;
        }
    }
    StringBuilder order = new();
    foreach (Stack<char> st in stacks!) order.Append(st.Pop());
    Console.WriteLine(order.ToString());
}
moves = ParseInput(read);
PartOne();
moves = null;

void PartTwo()
{
    Console.WriteLine("Part two:");
    Stack<char> inter = new();
    foreach ((int, int, int) move in moves)
    {
        int amnt = move.Item1;
        int from = move.Item2 - 1;
        int to = move.Item3 - 1;
        while (amnt > 0)
        {
            inter.Push(stacks![from].Pop());
            amnt--;
        }
        while (inter.Count > 0) stacks![to].Push(inter.Pop());
    }
    StringBuilder order = new();
    foreach (Stack<char> st in stacks!) order.Append(st.Pop());
    Console.WriteLine(order.ToString());
}
moves = ParseInput(read);
PartTwo();
