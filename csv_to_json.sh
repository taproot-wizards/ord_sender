#!/bin/bash

# Check if a file path is provided
if [ "$#" -ne 1 ]; then
  echo "Usage: $0 path_to_csv_file"
  exit 1
fi

# Assign the input file path to a variable
input_file="$1"

# Check if the file exists
if [ ! -f "$input_file" ]; then
  echo "File not found!"
  exit 1
fi

# Convert CSV to JSON using jq
csv_to_json() {
  local file="$1"

  # Ensure the input has a trailing newline to process the last line
  jq -R -s '
    split("\n") | map(select(length > 0)) | .[1:] |
    map(split(",") | {inscription_id: .[1], address: (.[2] | gsub("[\\r\\n]+"; ""))})
  ' <(cat "$file"; echo)
}

# Call the function and output the JSON
csv_to_json "$input_file"
