# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_amazeing_global_optspecs
	string join \n Z/zoom= C/colors= F/fps= P/show-perimeter h/help V/version
end

function __fish_amazeing_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_amazeing_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_amazeing_using_subcommand
	set -l cmd (__fish_amazeing_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c amazeing -n "__fish_amazeing_needs_command" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_needs_command" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_needs_command" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_needs_command" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_needs_command" -s V -l version -d 'Print version'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "create" -d 'Create a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "C" -d 'Create a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "view" -d 'View a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "V" -d 'View a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "solve" -d 'Solve a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "S" -d 'Solve a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "triangle"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "T"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "rectangle"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "R"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "hexagon"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "H"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "circle"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "C"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s b -l base -d 'Width of base of the triangle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from triangle" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s b -l base -d 'Width of base of the triangle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from T" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s r -l rows -d 'Number of rows' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s c -l cols -d 'Number of columns' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from rectangle" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s r -l rows -d 'Number of rows' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s c -l cols -d 'Number of columns' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from R" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s d -l diameter -d 'Width/Height of the circle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from hexagon" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s d -l diameter -d 'Width/Height of the circle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from H" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s d -l diameter -d 'Width/Height of the circle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from circle" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s d -l diameter -d 'Width/Height of the circle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from C" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from help" -f -a "triangle"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from help" -f -a "rectangle"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from help" -f -a "hexagon"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from help" -f -a "circle"
complete -c amazeing -n "__fish_amazeing_using_subcommand create; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "triangle"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "T"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "rectangle"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "R"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "hexagon"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "H"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "circle"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "C"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and not __fish_seen_subcommand_from triangle T rectangle R hexagon H circle C help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s b -l base -d 'Width of base of the triangle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from triangle" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s b -l base -d 'Width of base of the triangle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from T" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s r -l rows -d 'Number of rows' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s c -l cols -d 'Number of columns' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from rectangle" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s r -l rows -d 'Number of rows' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s c -l cols -d 'Number of columns' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from R" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s d -l diameter -d 'Width/Height of the circle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from hexagon" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s d -l diameter -d 'Width/Height of the circle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from H" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s d -l diameter -d 'Width/Height of the circle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from circle" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s d -l diameter -d 'Width/Height of the circle' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s u -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
octagon\t''
octagon-square\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s j -l jumble-factor -d 'Weight randomization factor (to use with AStar)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s w -l weight-direction -d 'Weight direction (ordering) (to use with AStar)' -r -f -a "forward\t''
backward\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from C" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from help" -f -a "triangle"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from help" -f -a "rectangle"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from help" -f -a "hexagon"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from help" -f -a "circle"
complete -c amazeing -n "__fish_amazeing_using_subcommand C; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s m -l maze -d 'Maze file path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s u -l update -d 'View and update'
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s h -l help -d 'Print help'
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s m -l maze -d 'Maze file path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s u -l update -d 'View and update'
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s h -l help -d 'Print help'
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s m -l maze -d 'Maze file path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s p -l procedure -d 'Maze Solving Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s v -l verbose -d 'Show a simulation of the solving process'
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s h -l help -d 'Print help'
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s m -l maze -d 'Maze file path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s p -l procedure -d 'Maze Solving Procedure' -r -f -a "bfs\t''
dfs\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s v -l verbose -d 'Show a simulation of the solving process'
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s P -l show-perimeter -d 'Draw maze bound (perimeter)'
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s h -l help -d 'Print help'
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and not __fish_seen_subcommand_from create view solve help" -f -a "create" -d 'Create a Maze'
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and not __fish_seen_subcommand_from create view solve help" -f -a "view" -d 'View a Maze'
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and not __fish_seen_subcommand_from create view solve help" -f -a "solve" -d 'Solve a Maze'
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and not __fish_seen_subcommand_from create view solve help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and __fish_seen_subcommand_from create" -f -a "triangle"
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and __fish_seen_subcommand_from create" -f -a "rectangle"
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and __fish_seen_subcommand_from create" -f -a "hexagon"
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and __fish_seen_subcommand_from create" -f -a "circle"
