bubble_sort_opt2() {  
  echo "bubble_sort_opt2"
  echo ${numbers[*]}

  start=0
  end=$((${#numbers[@]}-2))
    
  coherrent_end=${numbers[0]}
  
  outter_counter=0
  inner_counter=0

  sorted=0
  while [ $sorted -gt -1 ]
  do
      
    outter_counter=$((outter_counter+1))  
    echo "outter loop # $outter_counter"
    
    sorted=-1
    for index in `(seq $start $end)`      
    do  
      
      inner_counter=$((inner_counter+1))
    
      next_index=$((index+1)) 
      
      left=${numbers[$index]}
      right=${numbers[$next_index]}
      
      if [[ $left -gt $right ]];
      then 
        swap=$left
        numbers[$index]=$right
        numbers[$next_index]=$left
        
        if [[ be_sorting -eq 0 ]];
        then
          be_sorting=1
        fi
        
        if [[ $index -eq $start && $coherrent_end -gt $right ]]
        then
         start=$((start-1))
         if [[ $start -eq -1 ]];
         then
            start=0
         fi
        fi
        
        sorted=$index
        
        echo ${numbers[*]}                 
      elif [[ $((next_index-start)) -eq 1 ]]
      then
        start=$next_index
        coherrent_end=$right
      fi
    done
    
    end=$((sorted-1))
  done
  
  echo "inner loops $inner_counter"
  echo
}

bubble_sort_opt() {  
  echo "bubble_sort_opt"
  echo ${numbers[*]}

  start=0
  end=$((${#numbers[@]}-2))
  
  be_sorting=1
  coherrent_end=${numbers[0]}
  
  outter_counter=0
  inner_counter=0

  while [ $be_sorting -eq 1 ]
  do
  
    outter_counter=$((outter_counter+1))  
    echo "outter loop # $outter_counter"
    
    be_sorting=0  
    for index in `(seq $start $end)`      
    do  
      
      inner_counter=$((inner_counter+1))
    
      next_index=$((index+1)) 
      
      left=${numbers[$index]}
      right=${numbers[$next_index]}
      
      if [[ $left -gt $right ]];
      then 
        swap=$left
        numbers[$index]=$right
        numbers[$next_index]=$left
        
        if [[ be_sorting -eq 0 ]];
        then
          be_sorting=1
        fi
        
        if [[ $index -eq $start && $coherrent_end -gt $right ]]
        then
         start=$((start-1))
         if [[ $start -eq -1 ]];
         then
            start=0
         fi
        fi
        
        echo ${numbers[*]}                 
      elif [[ $((next_index-start)) -eq 1 ]]
      then
        start=$next_index
        coherrent_end=$right
      fi
    done
    end=$((end-1))    
  done
  
  echo "inner loops $inner_counter"
  echo
}

bubble_sort() {  
  echo "bubble_sort"
  echo ${numbers[*]}

  start=0
  end=$((${#numbers[@]}-2))
  
  be_sorting=1  
  
  outter_counter=0
  inner_counter=0

  while [ $be_sorting -eq 1 ]
  do
    
    outter_counter=$((outter_counter+1))  
    echo "outter loop # $outter_counter"
    
    be_sorting=0  
    for index in `(seq $start $end)`      
    do  
    
      inner_counter=$((inner_counter+1))
    
      next_index=$((index+1)) 
      
      left=${numbers[$index]}
      right=${numbers[$next_index]}
      
      if [[ $left -gt $right ]];
      then 
        swap=$left
        numbers[$index]=$right
        numbers[$next_index]=$left
        
        if [[ be_sorting -eq 0 ]];
        then
          be_sorting=1
        fi
        
        echo ${numbers[*]}                       
      fi
    done
    end=$((end-1))
    
  done
  
  echo "inner loops $inner_counter"
  echo
}

numbers=(1 2 3 4 5 6 7 8 9 10)
bubble_sort_opt2 numbers
numbers=(1 2 3 4 5 6 7 8 9 10)
bubble_sort_opt numbers
numbers=(1 2 3 4 5 6 7 8 9 10)
bubble_sort numbers

numbers=(5 4 3 2 1 6 7 8 9 10)
bubble_sort_opt2 numbers
numbers=(5 4 3 2 1 6 7 8 9 10)
bubble_sort_opt numbers
numbers=(5 4 3 2 1 6 7 8 9 10)
bubble_sort numbers

numbers=(2 1 4 3 6 5 8 7)
bubble_sort_opt2 numbers
numbers=(2 1 4 3 6 5 8 7)
bubble_sort_opt numbers
numbers=(2 1 4 3 6 5 8 7)
bubble_sort numbers

numbers=(6 5 4 3 2 1)
bubble_sort_opt2 numbers
numbers=(6 5 4 3 2 1)
bubble_sort_opt numbers
numbers=(6 5 4 3 2 1)
bubble_sort numbers

numbers=(1 2 3 4 5 10 9 8 6 7)
bubble_sort_opt2 numbers
numbers=(1 2 3 4 5 10 9 8 6 7)
bubble_sort_opt numbers
numbers=(1 2 3 4 5 10 9 8 6 7)
bubble_sort numbers

numbers=(5 6 3 4 1 2)
bubble_sort_opt2 numbers
numbers=(5 6 3 4 1 2)
bubble_sort_opt numbers
numbers=(5 6 3 4 1 2)
bubble_sort numbers

numbers=(4 5 7 6 11 8 1)
bubble_sort_opt2 numbers
numbers=(4 5 7 6 11 8 1)
bubble_sort_opt numbers
numbers=(4 5 7 6 11 8 1)
bubble_sort numbers

numbers=(99 13 18 6 7 11 8 1)
bubble_sort_opt2 numbers
numbers=(99 13 18 6 7 11 8 1)
bubble_sort_opt numbers
numbers=(99 13 18 6 7 11 8 1)
bubble_sort numbers
