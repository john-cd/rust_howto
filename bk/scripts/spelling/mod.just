set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

root := clean(source_directory() + "/../..")

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

## --- SPELL -----------------------------------------

# Check spelling in markdown
[unix]
@spell extra_param='':
  ./spellcheck.sh {{extra_param}}

[windows]
@spell extra_param='':
  echo "Spell check is not implemented for Windows!"
