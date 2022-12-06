using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Text;

// Define the input file; test for debug, real deal for release
#if DEBUG
string infile = "../../../test_inputs/06.txt";
#else
string infile = "../../../inputs/06.txt";
#endif

// Read the AOC input
using StreamReader r = File.OpenText(infile);
char[] read = new char[r.BaseStream.Length];
r.Read(read, 0, (int)r.BaseStream.Length);
List<bool> waste = new();
foreach (char rd in read) waste.Add(rd == '\n');
Queue<char> selection = new(14);

bool CheckNext(int current, int count)
{
    char next = read[current];
    while (selection.Contains(next)) selection.Dequeue();
    selection.Enqueue(next);
    return selection.Count == count;
}

int PartOne()
{
    int current = 0;
    while (!CheckNext(current, 4)) current += 1;
    return current + 1;
}

int PartTwo()
{
    int current = 0;
    while (!CheckNext(current, 14)) current += 1;
    return current + 1;
}

void Bench(int runcount = 100)
{
    Stopwatch timer = new();
    List<double> runs = new();
    for (int i = 0; i < runcount; i++)
    {
        timer.Start();
        PartOne();
        timer.Stop();
        runs.Add(timer.Elapsed.TotalMilliseconds);
        timer.Reset();
    }
    double total = 0;
    double shortest = double.MaxValue;
    double longest = double.MinValue;
    foreach (double r in runs)
    {
        total += r;
        shortest = shortest < r ? shortest : r;
        longest = longest > r ? longest : r;
    }
    var avg = total / runs.Count;
    Console.WriteLine($"Part One\r\nShortest: {shortest}\r\nLongest: {longest}\r\nAvg: {avg}\nFirst: {runs[0]}");
    Console.WriteLine("----------");
    runs.Clear();
    for (int i = 0; i < runcount; i++)
    {
        timer.Start();
        PartTwo();
        timer.Stop();
        runs.Add(timer.Elapsed.TotalMilliseconds);
        timer.Reset();
    }
    total = 0;
    shortest = double.MaxValue;
    longest = double.MinValue;
    foreach (double r in runs)
    {
        total += r;
        shortest = shortest < r ? shortest : r;
        longest = longest > r ? longest : r;
    }
    avg = total / runs.Count;
    Console.WriteLine($"Part Two\r\nShortest: {shortest}\r\nLongest: {longest}\r\nAvg: {avg}\nFirst: {runs[0]}");
    Console.WriteLine("----------");
}
Bench(1000);
Console.WriteLine(PartOne());
Console.WriteLine(PartTwo());
