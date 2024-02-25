#!/bin/bash

declare -i counter
declare -i input
read input

while [ $input -gt 0 ]
do        
    if [[ $((input % 2)) == 1 ]];
    then
        for((i=0; i < input; ++i))
        do
            echo -n ' *'
            counter=$((++counter))
        done
    fi          
        echo ' ' $input
        input=$((input/2))
done

echo 
echo Count $counter
