numbers=(90 13 4 17 22 8 53 66 1 3 100 2)
length=${#numbers[@]}
echo ${numbers[*]}

for sorted_index in `(seq 0 $((length - 2)))`
do
  minimum_index=$sorted_index  
  for sorting_index in `(seq $((sorted_index + 1)) $((length - 1)))`  
  do    
    if [[ ${numbers[$sorting_index]} -lt  ${numbers[$minimum_index]} ]]
    then minimum_index=$sorting_index
    fi    
  done
  swap=${numbers[$sorted_index]}
  numbers[$sorted_index]=${numbers[$minimum_index]}
  numbers[$minimum_index]=$swap
done  

echo ${numbers[*]}
