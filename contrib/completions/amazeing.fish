# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_amazeing_global_optspecs
	string join \n M/maze-shape= U/unit-shape= Z/zoom= C/colors= F/fps= h/help V/version
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

complete -c amazeing -n "__fish_amazeing_needs_command" -s M -l maze-shape -d 'Maze shape' -r -f -a "rectangle\t''"
complete -c amazeing -n "__fish_amazeing_needs_command" -s U -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
circle\t''"
complete -c amazeing -n "__fish_amazeing_needs_command" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_needs_command" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_needs_command" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_needs_command" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_needs_command" -s V -l version -d 'Print version'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "create" -d 'Create a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "C" -d 'Create a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "view" -d 'View a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "V" -d 'View a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "solve" -d 'Solve a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "S" -d 'Solve a Maze'
complete -c amazeing -n "__fish_amazeing_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s s -l source -d 'Starting point(s) of the generation' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s r -l rows -d 'Number of rows' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s c -l cols -d 'Number of cols' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s M -l maze-shape -d 'Maze shape' -r -f -a "rectangle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s U -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
circle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand create" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s m -l maze -d 'File path to dump Maze data' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s s -l source -d 'Starting point(s) of the generation' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s p -l procedure -d 'Maze Generation Procedure' -r -f -a "bfs\t''
dfs\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s r -l rows -d 'Number of rows' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s c -l cols -d 'Number of cols' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s M -l maze-shape -d 'Maze shape' -r -f -a "rectangle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s U -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
circle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s v -l verbose -d 'Show a simulation of the generation process'
complete -c amazeing -n "__fish_amazeing_using_subcommand C" -s h -l help -d 'Print help (see more with \'--help\')'
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s m -l maze -d 'Maze file path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s M -l maze-shape -d 'Maze shape' -r -f -a "rectangle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s U -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
circle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s u -l update -d 'View and update'
complete -c amazeing -n "__fish_amazeing_using_subcommand view" -s h -l help -d 'Print help'
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s m -l maze -d 'Maze file path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s M -l maze-shape -d 'Maze shape' -r -f -a "rectangle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s U -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
circle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s u -l update -d 'View and update'
complete -c amazeing -n "__fish_amazeing_using_subcommand V" -s h -l help -d 'Print help'
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s m -l maze -d 'Maze file path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s p -l procedure -d 'Maze Solving Procedure' -r -f -a "bfs\t''
dfs\t''
dijkstra\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s M -l maze-shape -d 'Maze shape' -r -f -a "rectangle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s U -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
circle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s v -l verbose -l verbose -d 'Show a simulation of the solving process'
complete -c amazeing -n "__fish_amazeing_using_subcommand solve" -s h -l help -d 'Print help'
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s m -l maze -d 'Maze file path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s p -l procedure -d 'Maze Solving Procedure' -r -f -a "bfs\t''
dfs\t''
dijkstra\t''
a-star\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s H -l heuristic-function -d 'Heuristic function (to use with AStar)' -r -f -a "manhattan\t''
euclidean\t''
chebyshev\t''
octile\t''
dijkstra\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s M -l maze-shape -d 'Maze shape' -r -f -a "rectangle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s U -l unit-shape -d 'Unit shape' -r -f -a "triangle\t''
square\t''
hexagon\t''
circle\t''"
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s Z -l zoom -d 'Display size (zoom)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s C -l colors -d 'Color file (.toml) path' -r -F
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s F -l fps -d 'Frame rate per second (controls simulation speed)' -r
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s v -l verbose -l verbose -d 'Show a simulation of the solving process'
complete -c amazeing -n "__fish_amazeing_using_subcommand S" -s h -l help -d 'Print help'
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and not __fish_seen_subcommand_from create view solve help" -f -a "create" -d 'Create a Maze'
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and not __fish_seen_subcommand_from create view solve help" -f -a "view" -d 'View a Maze'
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and not __fish_seen_subcommand_from create view solve help" -f -a "solve" -d 'Solve a Maze'
complete -c amazeing -n "__fish_amazeing_using_subcommand help; and not __fish_seen_subcommand_from create view solve help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
