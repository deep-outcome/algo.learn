insertion_sort() {

echo ${numbers[*]}

start=1
end=$((${#numbers[@]}-1))

for global in `(seq $start $end)`
do
  
  right=${numbers[$global]}
  
  local=$((global-1))
  while [ $local -gt -1 ]
  do
    left=${numbers[$local]}    
    
    if [[ $left -gt $right ]];
    then      
      numbers[$local]=$right
      numbers[$((local+1))]=$left
    else
      break
    fi    
    
    local=$((local-1))
  done  
done

echo ${numbers[*]}
echo
}

numbers=(15 12 13 7 9 10 2 14 19 11 22 1)
insertion_sort numbers

numbers=(99 12 88 7 9 33 2 14 33 11 22 1)
insertion_sort numbers
