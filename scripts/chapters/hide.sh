#! /bin/bash
set -euo pipefail

# `mdbook-private` is configured to hide markdown files starting with _ (and their children)
# This script hides some markdown sections/ pages by adding a _ prefix to all filenames listed in `hiddenfiles.txt`

function process() {
  ## $1 is the file being scanned
  ## Path to the directory containing the file being scanned
  dir=$(dirname "$1")
  ## $2 is the filename to update, $rel is the relative path of $2 with respect to the file being scanned, escaped (dot and slashes)
  rel=$(realpath --relative-to=$dir $2 | sed -e 's/[.]/\\./g' -e 's/[/]/\\\//g')
  #echo $rel
  escaped_filename=$(basename $2 | sed 's/\./\\\./g')
  #echo $escaped_filename
  echo "Scanning " $1 "for links containing " $rel
  ## Replace references to <filename> (e.g. in links) by <_filename>
  sed -r -i "/$rel/s/($escaped_filename)/_\1/g" $1
}

# Export function so it can be used by `bash -c` https://www.baeldung.com/linux/find-exec-command
export -f process

root="/code/"

for file in `cat $(dirname $0)/hiddenfiles.txt`
do
if [ -f $file ]; then
  echo "Hiding " $file
  # Rename the file to hide to <_filename>
  mv -v -- "$file" "$(dirname $file)/_$(basename $file)"
  # Scan all markdown files and update links with the new <_filename>
  find ${root}src -type f -name "*.md" -exec bash -c "process \"{}\" $file"  \;
fi
done

echo "DONE"
