#!/usr/bin/env bash

while true
do
  mem=$(free -m | awk 'NR==2{printf "%.2f%%\t\t", $3*100/$2 }')
  echo "Memory Usage: $mem"
  sleep 1
done