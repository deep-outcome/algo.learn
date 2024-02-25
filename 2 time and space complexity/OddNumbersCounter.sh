#!/bin/bash

declare -i counter
declare -i input
read input

for((i = 0; i <= input; ++i))
do        
    if [[ $((i % 2)) == 1 ]];
    then        
        echo $i
        counter=$((++counter))        
    fi
done

echo 
echo Count $counter
