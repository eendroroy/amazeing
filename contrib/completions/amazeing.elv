
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
            cand -B 'Block shape'
            cand --block-shape 'Block shape'
            cand -S 'Display size'
            cand --display-size 'Display size'
            cand -D 'Display density'
            cand --display-density 'Display density'
            cand -C 'Color scheme file (.toml) path'
            cand --color-scheme 'Color scheme file (.toml) path'
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
            cand -m 'File path to dump Maze data'
            cand --maze 'File path to dump Maze data'
            cand -s 'Starting point(s) of the generation'
            cand --source 'Starting point(s) of the generation'
            cand -p 'Maze Generation Procedure'
            cand --procedure 'Maze Generation Procedure'
            cand -r 'Number of rows'
            cand --rows 'Number of rows'
            cand -c 'Number of cols'
            cand --cols 'Number of cols'
            cand -f 'Simulation speed'
            cand --fps 'Simulation speed'
            cand -B 'Block shape'
            cand --block-shape 'Block shape'
            cand -S 'Display size'
            cand --display-size 'Display size'
            cand -D 'Display density'
            cand --display-density 'Display density'
            cand -C 'Color scheme file (.toml) path'
            cand --color-scheme 'Color scheme file (.toml) path'
            cand -v 'Show a simulation of the generation process'
            cand --verbose 'Show a simulation of the generation process'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
        }
        &'amazeing;C'= {
            cand -m 'File path to dump Maze data'
            cand --maze 'File path to dump Maze data'
            cand -s 'Starting point(s) of the generation'
            cand --source 'Starting point(s) of the generation'
            cand -p 'Maze Generation Procedure'
            cand --procedure 'Maze Generation Procedure'
            cand -r 'Number of rows'
            cand --rows 'Number of rows'
            cand -c 'Number of cols'
            cand --cols 'Number of cols'
            cand -f 'Simulation speed'
            cand --fps 'Simulation speed'
            cand -B 'Block shape'
            cand --block-shape 'Block shape'
            cand -S 'Display size'
            cand --display-size 'Display size'
            cand -D 'Display density'
            cand --display-density 'Display density'
            cand -C 'Color scheme file (.toml) path'
            cand --color-scheme 'Color scheme file (.toml) path'
            cand -v 'Show a simulation of the generation process'
            cand --verbose 'Show a simulation of the generation process'
            cand -h 'Print help (see more with ''--help'')'
            cand --help 'Print help (see more with ''--help'')'
        }
        &'amazeing;view'= {
            cand -m 'Maze file path'
            cand --maze 'Maze file path'
            cand -B 'Block shape'
            cand --block-shape 'Block shape'
            cand -S 'Display size'
            cand --display-size 'Display size'
            cand -D 'Display density'
            cand --display-density 'Display density'
            cand -C 'Color scheme file (.toml) path'
            cand --color-scheme 'Color scheme file (.toml) path'
            cand -u 'View and update'
            cand --update 'View and update'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'amazeing;V'= {
            cand -m 'Maze file path'
            cand --maze 'Maze file path'
            cand -B 'Block shape'
            cand --block-shape 'Block shape'
            cand -S 'Display size'
            cand --display-size 'Display size'
            cand -D 'Display density'
            cand --display-density 'Display density'
            cand -C 'Color scheme file (.toml) path'
            cand --color-scheme 'Color scheme file (.toml) path'
            cand -u 'View and update'
            cand --update 'View and update'
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
            cand -f 'Simulation speed'
            cand --fps 'Simulation speed'
            cand -B 'Block shape'
            cand --block-shape 'Block shape'
            cand -S 'Display size'
            cand --display-size 'Display size'
            cand -D 'Display density'
            cand --display-density 'Display density'
            cand -C 'Color scheme file (.toml) path'
            cand --color-scheme 'Color scheme file (.toml) path'
            cand -v 'Show a simulation of the generation process'
            cand --verbose 'Show a simulation of the generation process'
            cand --verbose 'Show a simulation of the generation process'
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
            cand -f 'Simulation speed'
            cand --fps 'Simulation speed'
            cand -B 'Block shape'
            cand --block-shape 'Block shape'
            cand -S 'Display size'
            cand --display-size 'Display size'
            cand -D 'Display density'
            cand --display-density 'Display density'
            cand -C 'Color scheme file (.toml) path'
            cand --color-scheme 'Color scheme file (.toml) path'
            cand -v 'Show a simulation of the generation process'
            cand --verbose 'Show a simulation of the generation process'
            cand --verbose 'Show a simulation of the generation process'
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
