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
    $len = count($row);
    $startright = $idx + 1;
    $over = $row[$idx] - 1;
    $from_left = $from_right = false;
    for ($i = 0; $i < $idx; $i++) {
        $from_left = $row[$i] > $over;
        if ($from_left) break;
    }
    for ($i = $startright; $i < $len; $i++) {
        $from_right = $row[$i] > $over;
        if ($from_right) break;
    }
    return ($from_left && $from_right) << 0;
}

$total = 0;
$visible = 0;
for ($y = 0; $y <= $cols; $y++) {
    $col = [];
    $hidden = [];
    $y_inbounds = $y > 0 && $y < $cols;
    for ($x = 0; $x <= $rows; $x++) {
        $total += 1;
        $cur = $input[$x][$y];
        array_push($col, $cur);
        if ($x === $rows) {
            foreach ($hidden as $idx => &$h) {
                if ($h === 1) $h = row_vis($idx, $col);
            }
        } elseif ($y_inbounds && $x > 0 && $x < $rows) {
            array_push($hidden, row_vis($y, $input[$x]));
        } else {
            array_push($hidden, 0);
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
