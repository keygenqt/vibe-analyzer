#!/bin/bash
## Module documentation for Bash testing
### This is also a module comment

# Regular comment - ignored

# Configuration variables
CONFIG_DIR="$HOME/.vibe-analyzer"
MAX_SIZE=1024
TIMEOUT=30

# Function to add two numbers
# Arguments: a b
# Returns: sum
add() {
    local a=$1
    local b=$2
    echo $((a + b))
}

# Function to multiply two numbers
multiply() {
    local a=$1
    local b=$2
    echo $((a * b))
}

# Function to check if file exists
# Arguments: path
# Returns: 0 if exists, 1 otherwise
file_exists() {
    if [ -f "$1" ]; then
        return 0
    else
        return 1
    fi
}

# Function with multiple lines
process_data() {
    local input=$1
    local output=""

    for i in $(seq 1 10); do
        output="$output $i"
    done

    echo "$output"
}

# Source external file
source ./utils.sh
. ./config.sh

# Regular comment at the end - ignored
