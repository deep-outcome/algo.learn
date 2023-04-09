numbers=(15 13 22 1 4 70 55 91 87 100 101 92)
echo ${numbers[*]}

corrected_length=$((${#numbers[@]}-2))
be_sorting=1

while_counter=0
for_counter=0

while [ $be_sorting -eq 1 ]
do
  
  while_counter=$((while_counter+1))

  be_sorting=0  
  for index in `(seq 0 $corrected_length)`  
  do    
  
    for_counter=$((for_counter+1)) 
    
    next_index=$((index+1))    
    if [[ ${numbers[$index]} -gt ${numbers[$next_index]} ]];
    then 
      swap=${numbers[$index]}
      numbers[$index]=${numbers[$next_index]}
      numbers[$next_index]=$swap
      
      if [[ be_sorting -eq 0 ]];
      then
        be_sorting=1
      fi
      
      echo ${numbers[*]}
    fi    
  done
  corrected_length=$((corrected_length-1))
done

echo $while_counter
echo $for_counter
