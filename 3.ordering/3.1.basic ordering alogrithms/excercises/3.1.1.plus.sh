numbers=(15 13 22 1 4 70 55 91 87 100 101 92)
echo ${numbers[*]}

length=$((${#numbers[@]}-2))
left_max_index=(($length-1))

while_counter=0
for_counter=0

while [ true ]
do
  
  while_counter=$((while_counter+1))

  sorted=-1
  for index in `(seq 0 $length)`  
  do        
    for_counter=$((for_counter+1))    
  
    next_index=$((index+1))    
    if [[ ${numbers[$index]} -gt ${numbers[$next_index]} ]];
    then 
      swap=${numbers[$index]}
      numbers[$index]=${numbers[$next_index]}
      numbers[$next_index]=$swap
      
      sorted=$index
      echo ${numbers[*]}
    fi    
  done
  
  if [[ $sorted -eq -1 ]];
  then
    break
  fi
  
  length=$((sorted-1))
done

echo $while_counter
echo $for_counter
