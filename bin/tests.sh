#!/bin/bash

function need_cmd() {
  if ! command -v $1 >/dev/null 2>&1; then
    echo "Need command to be available: $1 - consider running make init-<platform>" 1>&2
    return 2
  fi
}

if [[ -n $TRAVIS ]]; then
  echo "investigating travis"
  ls -la ~
  find / -name aapt -type f
  find / -name dx -type f
fi

for cmd in android keytool javac javadoc jarsigner dx zipalign aapt; do
  need_cmd $cmd || exit $?
done

AND=${1:?First argument must be the 'and' program path to use for testing}
echo "TBD with $AND"
exit 3