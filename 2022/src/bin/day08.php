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

$total = 0;
$visible = 0;
$maxscore = 0;
for ($y = 0; $y <= $cols; $y++) {
    $col = [];
    $hidden = [];
    $scores = [];
    $y_inbounds = $y > 0 && $y < $cols;
    for ($x = 0; $x <= $rows; $x++) {
        $total += 1;
        $cur = $input[$x][$y];
        array_push($col, $cur);
        if ($x === $rows) {
            foreach ($scores as $idx => &$s) {
                if ($s > 0) $s *= row_score($idx, $col);
                if ($hidden[$idx] === 1) $hidden[$idx] = row_vis($idx, $col);
            }
            $maxscore = max($maxscore, max($scores));
            $visible += array_sum($hidden);
        } elseif ($y_inbounds && $x > 0 && $x < $rows) {
            array_push($scores, row_score($y, $input[$x]));
            array_push($hidden, row_vis($y, $input[$x]));
        } else {
            array_push($scores, 0);
            array_push($hidden, 0);
        }
    }
}
$totaltime = microtime(true) - $start;
$processtime = $totaltime - $parsetime;
print("Part one result: " . $total - $visible . PHP_EOL);
print("Part two result: " . $maxscore . PHP_EOL);
$parsetime = 1000 * $parsetime;
$totaltime = 1000 * $totaltime;
$processtime = 1000 * $processtime;

print("Total runtime: $totaltime ms\n\tParsing: $parsetime ms\n\tProcessing: $processtime");
