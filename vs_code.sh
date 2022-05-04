#!/bin/bash

for LINE in $(cat .env)
do
  export "$LINE"
done

code .
