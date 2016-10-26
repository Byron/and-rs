#!/bin/bash

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

