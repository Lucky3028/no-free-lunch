#!/bin/bash
export $(grep -v '^#' .env | xargs -d '\n')
chmod +x no-free-lunch
DISCORD_TOKEN=$DISCORD_TOKEN APPLICATION_ID=$APPLICATION_ID ./no-free-lunch