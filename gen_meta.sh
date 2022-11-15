#!/bin/bash

if [[ $1 == clean ]];
then
    rm -f src/name.txt
    rm -f src/version.txt
    rm -f src/commit.txt
else
    printf $(awk -F ' = ' '$1 ~ /name/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml) > src/name.txt
    printf $(awk -F ' = ' '$1 ~ /version/ { gsub(/[\"]/, "", $2); printf("%s",$2) }' Cargo.toml) > src/version.txt
    printf $(git rev-parse --short HEAD) > src/commit.txt
fi