
use builtin;
use str;

set edit:completion:arg-completer[amazeing] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'amazeing'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'amazeing'= {
            cand -Z 'Display size (zoom)'
            cand --zoom 'Display size (zoom)'
            cand -C 'Color file (.toml) path'
            cand --colors 'Color file (.toml) path'
            cand -F 'Frame rate per second (controls simulation speed)'
            cand --fps 'Frame rate per second (controls simulation speed)'
            cand -P 'Draw maze bound (perimeter)'
            cand --show-perimeter 'Draw maze bound (perimeter)'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
            cand -V 'Print version'
            cand --version 'Print version'
            cand create 'Create a Maze'
            cand C 'Create a Maze'
            cand view 'View a Maze'
            cand V 'View a Maze'
            cand solve 'Solve a Maze'
            cand S 'Solve a Maze'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'amazeing;create'= {
            cand -u 'Unit shape'
            cand --unit-shape 'Unit shape'
            cand -m 'File path to dump Maze data'
            cand --maze 'File path to dump Maze data'
            cand -r 'Number of rows'
            cand --rows 'Number of rows'
            cand -c 'Number of columns'
            cand --cols 'Number of columns'
            cand -p 'Maze Generation Procedure'
            cand --procedure 'Maze Generation Procedure'
            cand -H 'Heuristic function (to use with AStar)'
            cand --heuristic-function 'Heuristic function (to use with AStar)'
            cand -j 'Weight randomization factor (to use with AStar)'
            cand --jumble-factor 'Weight randomization factor (to use with AStar)'
            cand -w 'Weight direction (ordering) (to use with AStar)'
            cand --weight-direction 'Weight direction (ordering) (to use with AStar)'
            cand -Z 'Display size (zoom)'
            cand --zoom 'Display size (zoom)'
            cand -C 'Color file (.toml) path'
            cand --colors 'Color file (.toml) path'
            cand -F 'Frame rate per second (controls simulation speed)'
            cand --fps 'Frame rate per second (controls simulation speed)'
            cand -v 'Show a simulation of the generation process'
            cand --verbose 'Show a simulation of the generation process'
            cand -P 'Draw maze bound (perimeter)'
            cand --show-perimeter 'Draw maze bound (perimeter)'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
        }
        &'amazeing;C'= {
            cand -u 'Unit shape'
            cand --unit-shape 'Unit shape'
            cand -m 'File path to dump Maze data'
            cand --maze 'File path to dump Maze data'
            cand -r 'Number of rows'
            cand --rows 'Number of rows'
            cand -c 'Number of columns'
            cand --cols 'Number of columns'
            cand -p 'Maze Generation Procedure'
            cand --procedure 'Maze Generation Procedure'
            cand -H 'Heuristic function (to use with AStar)'
            cand --heuristic-function 'Heuristic function (to use with AStar)'
            cand -j 'Weight randomization factor (to use with AStar)'
            cand --jumble-factor 'Weight randomization factor (to use with AStar)'
            cand -w 'Weight direction (ordering) (to use with AStar)'
            cand --weight-direction 'Weight direction (ordering) (to use with AStar)'
            cand -Z 'Display size (zoom)'
            cand --zoom 'Display size (zoom)'
            cand -C 'Color file (.toml) path'
            cand --colors 'Color file (.toml) path'
            cand -F 'Frame rate per second (controls simulation speed)'
            cand --fps 'Frame rate per second (controls simulation speed)'
            cand -v 'Show a simulation of the generation process'
            cand --verbose 'Show a simulation of the generation process'
            cand -P 'Draw maze bound (perimeter)'
            cand --show-perimeter 'Draw maze bound (perimeter)'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
        }
        &'amazeing;view'= {
            cand -m 'Maze file path'
            cand --maze 'Maze file path'
            cand -Z 'Display size (zoom)'
            cand --zoom 'Display size (zoom)'
            cand -C 'Color file (.toml) path'
            cand --colors 'Color file (.toml) path'
            cand -F 'Frame rate per second (controls simulation speed)'
            cand --fps 'Frame rate per second (controls simulation speed)'
            cand -u 'View and update'
            cand --update 'View and update'
            cand -P 'Draw maze bound (perimeter)'
            cand --show-perimeter 'Draw maze bound (perimeter)'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'amazeing;V'= {
            cand -m 'Maze file path'
            cand --maze 'Maze file path'
            cand -Z 'Display size (zoom)'
            cand --zoom 'Display size (zoom)'
            cand -C 'Color file (.toml) path'
            cand --colors 'Color file (.toml) path'
            cand -F 'Frame rate per second (controls simulation speed)'
            cand --fps 'Frame rate per second (controls simulation speed)'
            cand -u 'View and update'
            cand --update 'View and update'
            cand -P 'Draw maze bound (perimeter)'
            cand --show-perimeter 'Draw maze bound (perimeter)'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'amazeing;solve'= {
            cand -m 'Maze file path'
            cand --maze 'Maze file path'
            cand -p 'Maze Solving Procedure'
            cand --procedure 'Maze Solving Procedure'
            cand -H 'Heuristic function (to use with AStar)'
            cand --heuristic-function 'Heuristic function (to use with AStar)'
            cand -Z 'Display size (zoom)'
            cand --zoom 'Display size (zoom)'
            cand -C 'Color file (.toml) path'
            cand --colors 'Color file (.toml) path'
            cand -F 'Frame rate per second (controls simulation speed)'
            cand --fps 'Frame rate per second (controls simulation speed)'
            cand -v 'Show a simulation of the solving process'
            cand --verbose 'Show a simulation of the solving process'
            cand -P 'Draw maze bound (perimeter)'
            cand --show-perimeter 'Draw maze bound (perimeter)'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'amazeing;S'= {
            cand -m 'Maze file path'
            cand --maze 'Maze file path'
            cand -p 'Maze Solving Procedure'
            cand --procedure 'Maze Solving Procedure'
            cand -H 'Heuristic function (to use with AStar)'
            cand --heuristic-function 'Heuristic function (to use with AStar)'
            cand -Z 'Display size (zoom)'
            cand --zoom 'Display size (zoom)'
            cand -C 'Color file (.toml) path'
            cand --colors 'Color file (.toml) path'
            cand -F 'Frame rate per second (controls simulation speed)'
            cand --fps 'Frame rate per second (controls simulation speed)'
            cand -v 'Show a simulation of the solving process'
            cand --verbose 'Show a simulation of the solving process'
            cand -P 'Draw maze bound (perimeter)'
            cand --show-perimeter 'Draw maze bound (perimeter)'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'amazeing;help'= {
            cand create 'Create a Maze'
            cand view 'View a Maze'
            cand solve 'Solve a Maze'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'amazeing;help;create'= {
        }
        &'amazeing;help;view'= {
        }
        &'amazeing;help;solve'= {
        }
        &'amazeing;help;help'= {
        }
    ]
    $completions[$command]
}
