
use builtin;
use str;

set edit:completion:arg-completer[acai] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'acai'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'acai'= {
            cand -h 'Print help'
            cand --help 'Print help'
            cand install 'Installs packages'
            cand search 'Searches seeds'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'acai;install'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'acai;search'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'acai;help'= {
            cand install 'Installs packages'
            cand search 'Searches seeds'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'acai;help;install'= {
        }
        &'acai;help;search'= {
        }
        &'acai;help;help'= {
        }
    ]
    $completions[$command]
}
