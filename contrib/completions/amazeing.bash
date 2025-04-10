_amazeing() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in ${COMP_WORDS[@]}
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="amazeing"
                ;;
            amazeing,C)
                cmd="amazeing__create"
                ;;
            amazeing,S)
                cmd="amazeing__solve"
                ;;
            amazeing,V)
                cmd="amazeing__view"
                ;;
            amazeing,create)
                cmd="amazeing__create"
                ;;
            amazeing,help)
                cmd="amazeing__help"
                ;;
            amazeing,solve)
                cmd="amazeing__solve"
                ;;
            amazeing,view)
                cmd="amazeing__view"
                ;;
            amazeing__help,create)
                cmd="amazeing__help__create"
                ;;
            amazeing__help,help)
                cmd="amazeing__help__help"
                ;;
            amazeing__help,solve)
                cmd="amazeing__help__solve"
                ;;
            amazeing__help,view)
                cmd="amazeing__help__view"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        amazeing)
            opts="-B -S -D -C -h -V --block-shape --display-size --display-density --color-scheme --help --version create C view V solve S help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --block-shape)
                    COMPREPLY=($(compgen -W "square hexagon" -- "${cur}"))
                    return 0
                    ;;
                -B)
                    COMPREPLY=($(compgen -W "square hexagon" -- "${cur}"))
                    return 0
                    ;;
                --display-size)
                    COMPREPLY=($(compgen -W "xxs xs s m l xl xxl" -- "${cur}"))
                    return 0
                    ;;
                -S)
                    COMPREPLY=($(compgen -W "xxs xs s m l xl xxl" -- "${cur}"))
                    return 0
                    ;;
                --display-density)
                    COMPREPLY=($(compgen -W "connected dense standard cozy ample" -- "${cur}"))
                    return 0
                    ;;
                -D)
                    COMPREPLY=($(compgen -W "connected dense standard cozy ample" -- "${cur}"))
                    return 0
                    ;;
                --color-scheme)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        amazeing__create)
            opts="-m -s -p -r -c -v -f -B -S -D -C -h --maze --source --procedure --rows --cols --verbose --fps --block-shape --display-size --display-density --color-scheme --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --maze)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --source)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -s)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --procedure)
                    COMPREPLY=($(compgen -W "bfs dfs" -- "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -W "bfs dfs" -- "${cur}"))
                    return 0
                    ;;
                --rows)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -r)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --cols)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -c)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fps)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -f)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --block-shape)
                    COMPREPLY=($(compgen -W "square hexagon" -- "${cur}"))
                    return 0
                    ;;
                -B)
                    COMPREPLY=($(compgen -W "square hexagon" -- "${cur}"))
                    return 0
                    ;;
                --display-size)
                    COMPREPLY=($(compgen -W "xxs xs s m l xl xxl" -- "${cur}"))
                    return 0
                    ;;
                -S)
                    COMPREPLY=($(compgen -W "xxs xs s m l xl xxl" -- "${cur}"))
                    return 0
                    ;;
                --display-density)
                    COMPREPLY=($(compgen -W "connected dense standard cozy ample" -- "${cur}"))
                    return 0
                    ;;
                -D)
                    COMPREPLY=($(compgen -W "connected dense standard cozy ample" -- "${cur}"))
                    return 0
                    ;;
                --color-scheme)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        amazeing__help)
            opts="create view solve help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        amazeing__help__create)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        amazeing__help__help)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        amazeing__help__solve)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        amazeing__help__view)
            opts=""
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 3 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        amazeing__solve)
            opts="-m -p -H -v -t -B -S -D -C -h --maze --procedure --heuristic-function --verbose --verbose --tempo --block-shape --display-size --display-density --color-scheme --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --maze)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --procedure)
                    COMPREPLY=($(compgen -W "bfs dfs dijkstra a-star" -- "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -W "bfs dfs dijkstra a-star" -- "${cur}"))
                    return 0
                    ;;
                --heuristic-function)
                    COMPREPLY=($(compgen -W "manhattan euclidean chebyshev octile dijkstra" -- "${cur}"))
                    return 0
                    ;;
                -H)
                    COMPREPLY=($(compgen -W "manhattan euclidean chebyshev octile dijkstra" -- "${cur}"))
                    return 0
                    ;;
                --tempo)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -t)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --block-shape)
                    COMPREPLY=($(compgen -W "square hexagon" -- "${cur}"))
                    return 0
                    ;;
                -B)
                    COMPREPLY=($(compgen -W "square hexagon" -- "${cur}"))
                    return 0
                    ;;
                --display-size)
                    COMPREPLY=($(compgen -W "xxs xs s m l xl xxl" -- "${cur}"))
                    return 0
                    ;;
                -S)
                    COMPREPLY=($(compgen -W "xxs xs s m l xl xxl" -- "${cur}"))
                    return 0
                    ;;
                --display-density)
                    COMPREPLY=($(compgen -W "connected dense standard cozy ample" -- "${cur}"))
                    return 0
                    ;;
                -D)
                    COMPREPLY=($(compgen -W "connected dense standard cozy ample" -- "${cur}"))
                    return 0
                    ;;
                --color-scheme)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
        amazeing__view)
            opts="-m -u -B -S -D -C -h --maze --update --block-shape --display-size --display-density --color-scheme --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --maze)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --block-shape)
                    COMPREPLY=($(compgen -W "square hexagon" -- "${cur}"))
                    return 0
                    ;;
                -B)
                    COMPREPLY=($(compgen -W "square hexagon" -- "${cur}"))
                    return 0
                    ;;
                --display-size)
                    COMPREPLY=($(compgen -W "xxs xs s m l xl xxl" -- "${cur}"))
                    return 0
                    ;;
                -S)
                    COMPREPLY=($(compgen -W "xxs xs s m l xl xxl" -- "${cur}"))
                    return 0
                    ;;
                --display-density)
                    COMPREPLY=($(compgen -W "connected dense standard cozy ample" -- "${cur}"))
                    return 0
                    ;;
                -D)
                    COMPREPLY=($(compgen -W "connected dense standard cozy ample" -- "${cur}"))
                    return 0
                    ;;
                --color-scheme)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _amazeing -o nosort -o bashdefault -o default amazeing
else
    complete -F _amazeing -o bashdefault -o default amazeing
fi
