<?php

namespace Riven\AoC\Day8;

$input = [];
foreach (explode("\n", file_get_contents("../../inputs/08.txt", false, null)) as $line) {
    array_push($input, array_map('intval', str_split($line)));
}
// Remove empty line
array_pop($input);
$cols = count($input[0]) - 1;
$rows = count($input) - 1;

function row_vis(int $idx, array $row): int
{
    $rev = array_reverse($row);
    $len = count($row) - 1;
    $revidx = $len - $idx;
    $from_left = $from_right = false;
    for ($i = 0; $i <= $len; $i++) {
        // Stop checking either if it's true
        $from_left = $from_left || ($i < $idx && $row[$i] >= $row[$idx]);
        $from_right = $from_right || ($i < $revidx && $rev[$i] >= $row[$idx]);
        // Early exit from the loop once we know for sure
        if ($from_left && $from_right) return 1;
    }
    return 0;
}

$total = 0;
$visible = [];
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
    // For everything in this column not on an edge, check visibility
    for ($x = 1; $x < $rows; $x++) {
        if ($hidden[$x] == 1) $hidden[$x] = row_vis($x, $col);
    }
    array_push($visible, array_sum($hidden));
}
print("Part one result: " . $total - array_sum($visible) . PHP_EOL);

function row_score(int $idx, array $row): int
{
    $len = count($row) - 1;
    // There's always one tree visible unless we're at an edge
    $to_left = $to_right = 1;
    for ($x = $idx - 1; $x >= 0; $x--) {
        // Stop counting if we hit a tree as tall or taller than this
        if ($row[$x] >= $row[$idx] || $x == 0) break;
        // Otherwise we can see one more
        $to_left += 1;
    }
    // Same in the other direction
    for ($x = $idx + 1; $x <= $len; $x++) {
        if ($row[$x] >= $row[$idx] || $x == $len) break;
        $to_right += 1;
    }
    return $to_left * $to_right;
}

$scores = [];
for ($y = 0; $y <= $cols; $y++) {
    $col = [];
    $score = [];
    for ($x = 0; $x <= $rows; $x++) {
        $cur = $input[$x][$y];
        array_push($col, $cur);
        // Edges have one side without trees, so it always scores 0
        if ($y == 0 || $y == $cols || $x == 0 || $x == $rows) {
            array_push($score, 0);
        } else {
            array_push($score, row_score($y, $input[$x]));
        }
    }
    // For everything in this column, check score
    for ($x = 1; $x < $rows; $x++) {
        $score[$x] *= row_score($x, $col);
    }
    array_push($scores, max($score));
}
print("Part two result: " . max($scores));
