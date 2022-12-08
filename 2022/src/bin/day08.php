<?php

namespace Riven\AoC\Day8;

$intext = file_get_contents("../../inputs/08.txt", false, null);

// Get the runtime per part
$start = hrtime(true);

$input = [];
foreach (explode("\n", $intext) as $line) {
    array_push($input, array_map('intval', str_split($line)));
}
// Remove empty line
array_pop($input);
$cols = count($input[0]) - 1;
$rows = count($input) - 1;

$parsetime = hrtime(true) - $start;

function row_vis(int $idx, array $row): array
{
    $len = count($row);
    $maxidx = $len - 1;
    $over = $row[$idx] - 1;
    $from_left = $from_right = false;
    $to_left = $to_right = 1;
    for ($i = $idx - 1; $i > -1; $i--) {
        $from_left = $row[$i] > $over;
        if ($from_left) break;
        $to_left += 1 - ($i === 0);
    }
    for ($i = $idx + 1; $i < $len; $i++) {
        $from_right = $row[$i] > $over;
        if ($from_right) break;
        $to_right += 1 - ($i === $maxidx);
    }
    return [$to_left * $to_right, ($from_left && $from_right) << 0];
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
                $res = row_vis($idx, $col);
                if ($s > 0) $s *= $res[0];
                if ($hidden[$idx] === 1) $hidden[$idx] = $res[1];
            }
            $maxscore = max($maxscore, max($scores));
            $visible += array_sum($hidden);
        } elseif ($y_inbounds && $x > 0 && $x < $rows) {
            $res = row_vis($y, $input[$x]);
            array_push($scores, $res[0]);
            array_push($hidden, $res[1]);
        } else {
            array_push($scores, 0);
            array_push($hidden, 0);
        }
    }
}
$totaltime = hrtime(true) - $start;
$processtime = $totaltime - $parsetime;
print("Part one result: " . $total - $visible . PHP_EOL);
print("Part two result: " . $maxscore . PHP_EOL);
$parsetime = $parsetime;
$totaltime = $totaltime;
$processtime = $processtime;

print("Total runtime: $totaltime ns\n\tParsing: $parsetime ns\n\tProcessing: $processtime ns");
