#!/bin/bash
source "`dirname $0`/fix-travis.sh"

mode=${1:?First argument must be mode, one of: all, basic}
function need_cmd() {
  if ! command -v $1 >/dev/null 2>&1; then
    echo "Need command to be available: $1. $2" 1>&2
    return 2
  fi
}

for cmd in tr ; do
  need_cmd $cmd "This one is considered a standard tool" || exit $?
done

if [[ $mode = basic ]]; then
  exit 0
fi

for cmd in keytool javac javadoc jarsigner ; do
  need_cmd $cmd "Please be sure to have a working java installation" || exit $?
done

for cmd in android dx zipalign aapt; do
  need_cmd $cmd "Consider running make init-<platform>" || exit $?
done
