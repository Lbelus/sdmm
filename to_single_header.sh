#!/bin/bash

if [ -z "$1" ]; then
  echo "Usage: $0 <directory>"
  exit 1
fi

include_files=$(ls $1 | grep . | sed "s/^/#include </; s/$/>/")

echo -e "#ifndef __WRAPPER_HEADER_\n#define __WRAPPER_HEADER_\n\n$include_files\n\n#endif" > "$1/wrapper.h"
