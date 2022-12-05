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

(Stack<(int, int, int)>, List<Stack<char>>) Parse(char[] input)
{
    // State: parsing stacks or moves
    bool get_moves = false;

    // First step: parsing stacks
    // Check if we need to add more
    bool wrapped = false;
    // Current list to become stack
    int curstackidx = 0;
    // List to build stacks from
    List<List<char>> inputstacks = new();
    // List of stacks to output
    List<Stack<char>> outstacks = new();

    // Second step: parsing moves
    // amount of items to move
    int amnt = 0;
    // Stack to take from
    int from = 0;
    // Stack to move to
    int to = 0;
    // StringBuilder to feed into int.TryParse for move amounts
    StringBuilder num = new(3);
    // List of moves to store
    List<(int, int, int)> parsedmoves = new();
    for (int i = 0; i <= input.Length; i++)
    {
        if (i == input.Length)
        {
            parsedmoves.Insert(0, (amnt, from, to));
            break;
        }
        char cur = input[i];
        if (get_moves)
        {
            if (amnt > 0 && from > 0 && to > 0)
            {
                parsedmoves.Insert(0, (amnt, from, to));
                amnt = from = to = 0;
            }
            if (!char.IsNumber(cur))
            {
                if (num.Length > 0)
                {
                    if (amnt == 0 && int.TryParse(num.ToString(), out amnt)) num.Clear();
                    else if (from == 0 && int.TryParse(num.ToString(), out from)) num.Clear();
                    else if (to == 0 && int.TryParse(num.ToString(), out to)) num.Clear();
                }
                continue;
            }
            num.Append(cur);
        }
        else
        {
            if (char.IsNumber(cur))
            {
                foreach (var st in inputstacks) outstacks.Add(new(st));
                i += outstacks.Count * 4;
                get_moves = true;
                continue;
            }
            if (cur == '\n')
            {
                wrapped = true;
                curstackidx = 0;
                i += 1;
                continue;
            }
            bool increment = (i % 4) == 1;
            bool skip = char.IsWhiteSpace(cur) || cur == '[' || cur == ']';
            List<char> curstack;
            if (!wrapped && increment)
            {
                curstack = new();
                inputstacks.Add(curstack);
            }
            else if (skip)
            {
                if (increment) curstackidx += 1;
                continue;
            }
            else curstack = inputstacks[curstackidx];
            if (!skip) curstack.Insert(0, cur);
            curstackidx += 1;
        }
    }
    return (new(parsedmoves), outstacks);
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
            stacks[to].Push(stacks[from].Pop());
            amnt--;
        }
    }
    StringBuilder order = new();
    foreach (Stack<char> st in stacks!) order.Append(st.Pop());
    Console.WriteLine(order.ToString());
}
(moves, stacks) = Parse(read);
PartOne();

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
            inter.Push(stacks[from].Pop());
            amnt--;
        }
        while (inter.Count > 0) stacks[to].Push(inter.Pop());
    }
    StringBuilder order = new();
    foreach (Stack<char> st in stacks!) order.Append(st.Pop());
    Console.WriteLine(order.ToString());
}
(moves, stacks) = Parse(read);
PartTwo();
