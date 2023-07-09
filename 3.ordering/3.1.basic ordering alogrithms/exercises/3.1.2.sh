bubble_sort() {  
  echo ${numbers[*]}
  
  end=$((${#numbers[@]}-2))
  be_sorting=1

  counter=1

  while [ $be_sorting -eq 1 ]
  do
    echo "Outter loop num $counter"
    be_sorting=0  
    for index in `(seq 0 $end)`  
    do    
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
    end=$((end-1))
    counter=$((counter+1))
  done
  
  echo
}

numbers=(1 2 3 4 5 6 7 8 9 10)
bubble_sort numbers

numbers=(2 1 4 3 6 5 8 7)
bubble_sort numbers

numbers=(5 4 3 2 1)
bubble_sort numbers
