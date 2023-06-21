#!/bin/sh
set -euC

usage () {
    cat <<EOF
Usage: $0 --language-dir DIR --output-file FILE --topiary-wrapped FILE

  --help                     Show this help and quit
  --language-dir DIR         Where the language files are installed
  --output-file FILE         Where to put the generated wrapper
  --topiary-wrapped FILE     Where the wrapped Topiary binary is installed
EOF
}

die () {
    printf >&2 "$@"
    printf '\n'
    usage
    exit 2
}

language_dir=
output_file=
topiary_wrapped=

while [ $# -gt 0 ]; do
    arg=$1
    shift

    case $arg in
        --language-dir)
            language_dir=$1
            shift
            ;;

        --output-file)
            output_file=$1
            shift
            ;;

        --topiary-wrapped)
            topiary_wrapped=$1
            shift
            ;;

        --help)
            usage
            exit 0
            ;;

        *)
            die 'Error: I do not know what to do with argument `%s`.\n' "$arg"
    esac
done

[ -z "$language_dir" ] && die 'Error: You need to specify --language-dir.\n'
[ -z "$output_file" ] && die 'Error: You need to specify --output-file.\n'
[ -z "$topiary_wrapped" ] && die 'Error: You need to specify --topiary-wrapped.\n'

[ "${language_dir% *}" != "$language_dir" ] && die 'Error: --language-dir cannot contain spaces.\n'
[ "${topiary_wrapped% *}" != "$topiary_wrapped" ] && die 'Error: --topiary-wrapped cannot contain spaces.\n'

cat > "$output_file" <<EOF
#!/bin/sh
set -euC

## If \$TOPIARY_LANGUAGE_DIR is set, then keep it (even if it is null). If it is
## unset, default to $language_dir.
export TOPIARY_LANGUAGE_DIR=\${TOPIARY_LANGUAGE_DIR-$language_dir}

exec $topiary_wrapped "\$@"
EOF

chmod +x "$output_file"
