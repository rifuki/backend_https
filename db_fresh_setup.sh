#!/bin/bash

# clear console
clear

# loading animate
loading() {
  chars="/-\|"
  iterations=$1
  echo -n "Loading... "
  for (( i=0; i < $iterations; i++ )); do
      for (( j=0; j<${#chars}; j++ )); do
          printf "%s" "${chars:$j:1}"
          sleep 0.1 
          printf "\b"
      done
  done
  echo -n -e "\n"
}

# import .env file
if [ -f .env ]; then
  echo -e "\033[1m.env\033[0m File Found!\n"
  source .env
else
  echo "Please Set .env First!"
  exit 1
fi

# run docker-compose
docker_compose() {
  if [ -f docker-compose.yml  ]; then
    echo "Running Docker Compose File!"
    docker-compose down
    docker-compose up -d
  else
    echo "Please Set docker-compose.yml First!"
    exit 1
  fi
}

# run migrations
migration() {
  if [ -e migrations ]; then
    sqlx database drop -y
    sqlx database create
    sqlx migrate run
  else
    echo "Please Create SQLX Migrations File First!"
    exit 1
  fi
}

start() {
  docker_compose
  echo -e "\nRunning SQLX Migrations File!"
  loading 30
  migration
  echo -e "\033[1m✨Done✨\033[0m"
}

case "$1" in 
  start)
    start
    ;;
  compose)
    docker_compose
    ;;
  migrate)
    echo -e "\nRunning SQLX Migrations File!\n"
    migration
    ;;
  *)
    echo -e "\n\033[1mUsage: $0 [start|compose|migrate]\033[0m"
    exit 1
    ;;
esac

exit 0
