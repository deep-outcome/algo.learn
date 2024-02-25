#!/bin/bash


declare -i counter
declare -i input
read input

while [ $input -ge 1 ]
do        
    echo -n ' *'      
    input=$((input/2))    
    counter=$((++counter))
done

echo 
echo Count $counter
