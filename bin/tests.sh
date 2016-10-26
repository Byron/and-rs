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

function it () {
  local description=${1:?Need description text}
  local test_function=${2:?Need function to execute}
  local result
  echo -n "$description"
  local result
  result=`eval $test_function 2>&1`
  if [ $? = 0 ]; then
    echo " - OK" 1>&2
  else
    echo " - failed" 1>&2
    echo $result 1>&2
    exit 2
  fi
}

AND=${1:?First argument must be the 'and' program path to use for testing}

function test_it() {
  echo "this works"
  echo "this works on stderr" 1>&2
  return 0
}

it 'can execute simple tests' test_it
# it 'can create scaffolding for the most basic android application' test_create
