_amazeing() {
    local i cur prev opts cmd
    COMPREPLY=()
    if [[ "${BASH_VERSINFO[0]}" -ge 4 ]]; then
        cur="$2"
    else
        cur="${COMP_WORDS[COMP_CWORD]}"
    fi
    prev="$3"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]:0:COMP_CWORD}"
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
            opts="-Z -C -F -P -h -V --zoom --colors --fps --show-perimeter --help --version create C view V solve S help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --zoom)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -Z)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --colors)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fps)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -F)
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
            opts="-u -m -r -c -p -H -j -w -v -Z -C -F -P -h --unit-shape --maze --rows --cols --procedure --heuristic-function --jumble-factor --weight-direction --verbose --zoom --colors --fps --show-perimeter --help"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 2 ]] ; then
                COMPREPLY=( $(compgen -W "${opts}" -- "${cur}") )
                return 0
            fi
            case "${prev}" in
                --unit-shape)
                    COMPREPLY=($(compgen -W "triangle square hexagon hexagon-rectangle octagon octagon-square" -- "${cur}"))
                    return 0
                    ;;
                -u)
                    COMPREPLY=($(compgen -W "triangle square hexagon hexagon-rectangle octagon octagon-square" -- "${cur}"))
                    return 0
                    ;;
                --maze)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -m)
                    COMPREPLY=($(compgen -f "${cur}"))
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
                --procedure)
                    COMPREPLY=($(compgen -W "bfs dfs a-star" -- "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -W "bfs dfs a-star" -- "${cur}"))
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
                --jumble-factor)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -j)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --weight-direction)
                    COMPREPLY=($(compgen -W "forward backward" -- "${cur}"))
                    return 0
                    ;;
                -w)
                    COMPREPLY=($(compgen -W "forward backward" -- "${cur}"))
                    return 0
                    ;;
                --zoom)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -Z)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --colors)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fps)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -F)
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
            opts="-m -p -H -v -Z -C -F -P -h --maze --procedure --heuristic-function --verbose --zoom --colors --fps --show-perimeter --help"
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
                    COMPREPLY=($(compgen -W "bfs dfs a-star" -- "${cur}"))
                    return 0
                    ;;
                -p)
                    COMPREPLY=($(compgen -W "bfs dfs a-star" -- "${cur}"))
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
                --zoom)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -Z)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --colors)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fps)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -F)
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
            opts="-m -u -Z -C -F -P -h --maze --update --zoom --colors --fps --show-perimeter --help"
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
                --zoom)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -Z)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --colors)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -C)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                --fps)
                    COMPREPLY=($(compgen -f "${cur}"))
                    return 0
                    ;;
                -F)
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
