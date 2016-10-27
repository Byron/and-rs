#!/bin/bash
ROOT_DIR="`dirname $0`"
source "$ROOT_DIR/fix-travis.sh"

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
    echo "--- output stdout/stderr ---"
    echo $result 1>&2
    exit 2
  fi
}

function files_exists () {
  local res=0
  while read file_path; do
    if ! [ -f $file_path ]; then
      echo "file at $ROOT_DIR/$file_path does not exist"
      res=1
    fi
  done
  return $res
}

AND=${1:?First argument must be the 'and' program path to use for testing}
PROJECT_NAME=HelloAndroid
PACKAGE_NAME=com.company.package
REQUIRED_CREATE_ARGS=--package=$PACKAGE_NAME

function test_create_project_wrong_name () {
  $AND new "project/\$must-be-asciiWithoutDashesOrNonWordCharacters" $REQUIRED_CREATE_ARGS && return 1
  return 0
}

function test_create_project () {
  rm -Rf $PROJECT_NAME >/dev/null 2>&1
  $AND new "$PROJECT_NAME" $REQUIRED_CREATE_ARGS || return $?

  local package_path
  package_path=`echo $PACKAGE_NAME | tr . /` || return $?
  local manifest_file=$PROJECT_NAME/AndroidManifest.xml
  files_exists <<-FILES
    $PROJECT_NAME/src/$package_path/$PROJECT_NAME.java
    $manifest_file
FILES
}

it 'rejects non-ascii project names' test_create_project_wrong_name
it 'can create scaffolding for the most basic android application' test_create_project
