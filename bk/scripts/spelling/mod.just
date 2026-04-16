set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## --- SPELL -----------------------------------------

# Check spelling in markdown
[unix]
@spell extra_param='list':
  ./spellcheck.sh {{extra_param}}

[windows]
@spell extra_param='':
  echo "Spell check is not implemented for Windows!"

[unix]
add word:
  #!/usr/bin/env bash
  dic={{join(source_directory(), "dictionary.txt")}};
  echo "{{word}}" >> ${dic}
  sort -u -o ${dic} ${dic}
  sed -i '/personal_ws-1.1 en 0 utf-8/d; 1s/^/personal_ws-1.1 en 0 utf-8\n/' ${dic}
