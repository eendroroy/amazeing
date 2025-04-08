
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'amazeing' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'amazeing'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'amazeing' {
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('--display-size', '--display-size', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('--display-density', '--display-density', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('--color-scheme', '--color-scheme', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('create', 'create', [CompletionResultType]::ParameterValue, 'Create a Maze')
            [CompletionResult]::new('C', 'C', [CompletionResultType]::ParameterValue, 'Create a Maze')
            [CompletionResult]::new('view', 'view', [CompletionResultType]::ParameterValue, 'View a Maze')
            [CompletionResult]::new('V', 'V', [CompletionResultType]::ParameterValue, 'View a Maze')
            [CompletionResult]::new('solve', 'solve', [CompletionResultType]::ParameterValue, 'Solve a Maze')
            [CompletionResult]::new('S', 'S', [CompletionResultType]::ParameterValue, 'Solve a Maze')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'amazeing;create' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'File path to dump Maze data')
            [CompletionResult]::new('--maze', '--maze', [CompletionResultType]::ParameterName, 'File path to dump Maze data')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Starting point(s) of the generation')
            [CompletionResult]::new('--source', '--source', [CompletionResultType]::ParameterName, 'Starting point(s) of the generation')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Maze Generation Procedure')
            [CompletionResult]::new('--procedure', '--procedure', [CompletionResultType]::ParameterName, 'Maze Generation Procedure')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Number of rows')
            [CompletionResult]::new('--rows', '--rows', [CompletionResultType]::ParameterName, 'Number of rows')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Number of cols')
            [CompletionResult]::new('--cols', '--cols', [CompletionResultType]::ParameterName, 'Number of cols')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Simulation speed')
            [CompletionResult]::new('--tempo', '--tempo', [CompletionResultType]::ParameterName, 'Simulation speed')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('--display-size', '--display-size', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('--display-density', '--display-density', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('--color-scheme', '--color-scheme', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            break
        }
        'amazeing;C' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'File path to dump Maze data')
            [CompletionResult]::new('--maze', '--maze', [CompletionResultType]::ParameterName, 'File path to dump Maze data')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Starting point(s) of the generation')
            [CompletionResult]::new('--source', '--source', [CompletionResultType]::ParameterName, 'Starting point(s) of the generation')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Maze Generation Procedure')
            [CompletionResult]::new('--procedure', '--procedure', [CompletionResultType]::ParameterName, 'Maze Generation Procedure')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Number of rows')
            [CompletionResult]::new('--rows', '--rows', [CompletionResultType]::ParameterName, 'Number of rows')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Number of cols')
            [CompletionResult]::new('--cols', '--cols', [CompletionResultType]::ParameterName, 'Number of cols')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Simulation speed')
            [CompletionResult]::new('--tempo', '--tempo', [CompletionResultType]::ParameterName, 'Simulation speed')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('--display-size', '--display-size', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('--display-density', '--display-density', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('--color-scheme', '--color-scheme', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            break
        }
        'amazeing;view' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Maze file path')
            [CompletionResult]::new('--maze', '--maze', [CompletionResultType]::ParameterName, 'Maze file path')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('--display-size', '--display-size', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('--display-density', '--display-density', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('--color-scheme', '--color-scheme', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'View and update')
            [CompletionResult]::new('--update', '--update', [CompletionResultType]::ParameterName, 'View and update')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'amazeing;V' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Maze file path')
            [CompletionResult]::new('--maze', '--maze', [CompletionResultType]::ParameterName, 'Maze file path')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('--display-size', '--display-size', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('--display-density', '--display-density', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('--color-scheme', '--color-scheme', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'View and update')
            [CompletionResult]::new('--update', '--update', [CompletionResultType]::ParameterName, 'View and update')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'amazeing;solve' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Maze file path')
            [CompletionResult]::new('--maze', '--maze', [CompletionResultType]::ParameterName, 'Maze file path')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Maze Solving Procedure')
            [CompletionResult]::new('--procedure', '--procedure', [CompletionResultType]::ParameterName, 'Maze Solving Procedure')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'Heuristic function (to use with AStar)')
            [CompletionResult]::new('--heuristic-function', '--heuristic-function', [CompletionResultType]::ParameterName, 'Heuristic function (to use with AStar)')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Simulation speed')
            [CompletionResult]::new('--tempo', '--tempo', [CompletionResultType]::ParameterName, 'Simulation speed')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('--display-size', '--display-size', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('--display-density', '--display-density', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('--color-scheme', '--color-scheme', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'amazeing;S' {
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Maze file path')
            [CompletionResult]::new('--maze', '--maze', [CompletionResultType]::ParameterName, 'Maze file path')
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Maze Solving Procedure')
            [CompletionResult]::new('--procedure', '--procedure', [CompletionResultType]::ParameterName, 'Maze Solving Procedure')
            [CompletionResult]::new('-H', '-H ', [CompletionResultType]::ParameterName, 'Heuristic function (to use with AStar)')
            [CompletionResult]::new('--heuristic-function', '--heuristic-function', [CompletionResultType]::ParameterName, 'Heuristic function (to use with AStar)')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Simulation speed')
            [CompletionResult]::new('--tempo', '--tempo', [CompletionResultType]::ParameterName, 'Simulation speed')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('--display-size', '--display-size', [CompletionResultType]::ParameterName, 'Display size')
            [CompletionResult]::new('-D', '-D ', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('--display-density', '--display-density', [CompletionResultType]::ParameterName, 'Display density')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('--color-scheme', '--color-scheme', [CompletionResultType]::ParameterName, 'Color scheme file (.toml) path')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Show a simulation of the generation process')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'amazeing;help' {
            [CompletionResult]::new('create', 'create', [CompletionResultType]::ParameterValue, 'Create a Maze')
            [CompletionResult]::new('view', 'view', [CompletionResultType]::ParameterValue, 'View a Maze')
            [CompletionResult]::new('solve', 'solve', [CompletionResultType]::ParameterValue, 'Solve a Maze')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'amazeing;help;create' {
            break
        }
        'amazeing;help;view' {
            break
        }
        'amazeing;help;solve' {
            break
        }
        'amazeing;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
