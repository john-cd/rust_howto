set shell := ["bash", "-uc"]

# Set shell for Windows OSs:
# PowerShell Core has some of the Unix shell skills we need, such as || and &&
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

[no-exit-message]
_default:
  @just --list --unsorted --justfile {{source_file()}}

#@faa:
#  just bk fa

mod bk
mod playground
mod publish
mod tools
mod xmpl
