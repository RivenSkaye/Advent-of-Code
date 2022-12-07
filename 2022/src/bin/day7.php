<?php

namespace Riven\AoC\Day7;

$input = explode("\n", file_get_contents("../../inputs/07.txt", false, null, 7));
$dirs = array("size" => 0);
$current = &$dirs;

// Parse the file
foreach ($input as $line) {
    // First line is always `$ cd /` so we start by going into /
    $words = explode(" ", trim($line));
    if ($words[0] === "$") {
        $cmd = end($words);
        // The only commands used are `cd <dir>` and `ls` so we can be naïve.
        switch ($cmd) {
            case "ls":
                break;
            case "..":
                $current = &$current["parent"];
                break;
            case "/":
                $current = &$dirs;
                break;
            default:
                $current = &$current[$cmd];
                break;
        }
    } else {
        if (is_numeric($words[0])) {
            $current["size"] += intval($words[0]);
        } else {
            $current[end($words)] = array("size" => 0, "parent" => &$current);
        }
    }
}

function partOne(array $root, array &$outputs, &$all): int
{
    $totalsize = 0;
    foreach ($root as $dir => $content) {
        if ($dir === "size") {
            $totalsize += $content;
        } elseif ($dir !== "parent") {
            $totalsize += partOne($content, $outputs, $all);
        }
    }

    if ($totalsize <= 100000) {
        array_push($outputs, $totalsize);
    }
    array_push($all, $totalsize);

    return $totalsize;
}

$results = [];
$alldirs = [];
$fs_used = partOne($dirs, $results, $alldirs);
print(array_sum($results));
print(PHP_EOL);

function partTwo(array &$dirsizes, int $used): int
{
    $needed = 30000000 - (70000000 - $used);
    array_multisort($dirsizes);
    foreach ($dirsizes as $yeetable) {
        if ($yeetable >= $needed) {
            return $yeetable;
        }
    }
    return 0;
}
print(partTwo($alldirs, $fs_used));
