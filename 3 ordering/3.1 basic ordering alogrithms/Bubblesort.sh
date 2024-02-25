numbers=(15 13 22 1 4 70 55 91 87 100 101 92)
echo ${numbers[*]}

corrected_length=$((${#numbers[@]}-2))
be_sorting=1

while [ $be_sorting -eq 1 ]
do
  be_sorting=0
  for index in `(seq 0 $corrected_length)`  
  do    
    next_index=$((index+1))    
    if [[ ${numbers[$index]} -gt ${numbers[$next_index]} ]];
    then 
      swap=${numbers[$index]}
      numbers[$index]=${numbers[$next_index]}
      numbers[$next_index]=$swap      
      
      echo ${numbers[*]}
      if [[ be_sorting -eq 0 ]];
      then
        be_sorting=1
      fi
    fi    
  done  
done
