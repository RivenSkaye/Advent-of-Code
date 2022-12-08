<?php

namespace Riven\AoC\Day8;

$intext = file_get_contents("../../inputs/08.txt", false, null);

// Get the runtime per part
$start = microtime(true);

$input = [];
foreach (explode("\n", $intext) as $line) {
    array_push($input, array_map('intval', str_split($line)));
}
// Remove empty line
array_pop($input);
$cols = count($input[0]) - 1;
$rows = count($input) - 1;

$parsetime = microtime(true) - $start;

function row_vis(int $idx, array $row): int
{
    $rev = array_reverse($row);
    $len = count($row);
    $revidx = ($len - $idx) - 1;
    $over = $row[$idx] - 1;
    $from_left = $from_right = false;
    for ($i = 0; $i < $len; $i++) {
        // Stop checking either if it's true
        $from_left = $from_left || ($i < $idx && $row[$i] > $over);
        $from_right = $from_right || ($i < $revidx && $rev[$i] > $over);
        // Early exit from the loop once we know for sure
        if ($from_left && $from_right) return 1;
    }
    return 0;
}

$total = 0;
$visible = 0;
for ($y = 0; $y <= $cols; $y++) {
    $col = [];
    $hidden = [];
    for ($x = 0; $x <= $rows; $x++) {
        $total += 1;
        $cur = $input[$x][$y];
        array_push($col, $cur);
        // Push whether or not it's hidden on this row and column.
        // If either is 0, it's not hidden by default.
        if ($y > 0 && $y < $cols && $x > 0 && $x < $rows) {
            array_push($hidden, row_vis($y, $input[$x]));
        } else {
            array_push($hidden, 0);
        }
    }
    if ($y > 0 && $y < $cols) {
        // For everything in this column not on an edge, check visibility
        for ($x = 1; $x < $rows; $x++) {
            $chk = &$hidden[$x];
            if ($chk === 1) $chk = row_vis($x, $col);
        }
    }
    $visible += array_sum($hidden);
}
$p1time = (microtime(true) - $start) - $parsetime;
print("Part one result: " . $total - $visible . PHP_EOL);

function row_score(int $idx, array $row): int
{
    $len = count($row) - 1;
    // There's always one tree visible unless we're at an edge
    $to_left = $to_right = 1;
    for ($x = $idx - 1; $x > -1; $x--) {
        // Stop counting if we hit a tree as tall or taller than this
        if ($row[$x] >= $row[$idx] || $x === 0) break;
        // Otherwise we can see one more
        $to_left += 1;
    }
    // Same in the other direction
    for ($x = $idx + 1; $x < $len; $x++) {
        if ($row[$x] >= $row[$idx]) break;
        $to_right += 1;
    }
    return $to_left * $to_right;
}

$maxscore = 0;
for ($y = 0; $y <= $cols; $y++) {
    $col = [];
    $score = [];
    for ($x = 0; $x <= $rows; $x++) {
        $cur = $input[$x][$y];
        array_push($col, $cur);
        if ($x === $rows) {
            foreach ($score as $idx => &$s) {
                $s *= row_score($idx, $col);
            }
            $maxscore = max($maxscore, max($score));
        } elseif ($x === $rows || $y === 0 || $y === $cols || $x === 0) {
            // Edges have one side without trees, so it always scores 0
            array_push($score, 0);
        } else {
            array_push($score, row_score($y, $input[$x]));
        }
    }
}
$totaltime = microtime(true) - $start;
$p2time = ($totaltime - $parsetime) - $p1time;
$p1time = 1000 * $p1time;
$p2time = 1000 * $p2time;
$parsetime = 1000 * $parsetime;
$totaltime = 1000 * $totaltime;
print("Part two result: " . $maxscore . PHP_EOL);

print("Total runtime: $totaltime ms\n\tParsing: $parsetime ms\n\tP1: $p1time ms\tP2: $p2time ms.");
