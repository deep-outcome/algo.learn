#!/bin/bash

read input
declare -i counter

for((i = 1; i <= $input; ++i))
do
    for((j = 1; j <= $input; ++j, ++counter))
    do
        echo -n ' *'        
    done
done

echo 
echo Count $counter

