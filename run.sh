#!/bin/bash

cargo build 
cd lichess-bot
source venv/bin/activate
python3 lichess-bot.py -u -vvvv