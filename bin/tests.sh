#!/bin/bash

function need_cmd() {
  if ! command -v $1 >/dev/null 2>&1; then
    echo "Need command to be available: $1" 1>&2
    return 2
  fi
}

for cmd in android keytool aapt javac javadoc dx jarsigner zipalign; do
  need_cmd $cmd || exit $?
done

AND=${1:?First argument must be the 'and' program path to use for testing}
echo "TBD with $AND"
exit 3