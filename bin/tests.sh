#!/bin/bash

function need_cmd() {
  if ! command -v $1 >/dev/null 2>&1; then
    echo "Need command to be available: $1 - consider running make init-<platform>" 1>&2
    return 2
  fi
}

function find_build_tools_dir() {
  find /usr/local -name dx -type f 2>/dev/null | while read p; do
    echo "`dirname $p`"
    return 0
  done
  return 2
}

if [[ -n $TRAVIS ]]; then
  build_tools=`find_build_tools_dir`
  export PATH=$build_tools:$PATH
fi

for cmd in android keytool javac javadoc jarsigner dx zipalign aapt; do
  need_cmd $cmd || exit $?
done

AND=${1:?First argument must be the 'and' program path to use for testing}
echo "TBD with $AND"
exit 3