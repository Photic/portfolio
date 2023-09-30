#!/bin/bash

# Function to kill processes by name
kill_processes() {
  local process_name="portfolio"
  local pids=$(pgrep -f "$process_name")
  
  for pid in $pids; do
    kill -TERM $pid
  done
}

trap 'pkill -f "cargo-watch"; kill_processes; exit' INT

# Define your commands here
command1="(cargo watch --ignore 'src/generated/*' --shell 'RUST_LOG=debug cargo run')"
command2="(npx tailwindcss -i ./styles.css -o ./static/styles.css --watch)"

# Run the commands in the background
eval "$command1" &

# Run the second command in the foreground and then disown it
eval "$command2"
disown

# Wait for all background jobs to complete
wait
