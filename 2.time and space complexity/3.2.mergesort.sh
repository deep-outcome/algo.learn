
# problems with slices concatenation into per number array-item array

left_block=()
right_block=()

merged_block=()

merge (){

    merged_block=()

    left_length=${#left_block[*]}
    right_length=${#right_block[*]}
    
    echo "rb_m" ${right_block[*]}
    echo "lb_m" ${left_block[*]}
    
    echo "rl" $right_length
      echo "ll" $left_length
    
    left_index=0
    right_index=0
    
    #
    while [ $left_index -lt $left_length -a $right_index -lt $right_length ]
    do
      left=${left_block[left_index]}
      right=${right_block[right_index]}      
      
      echo "right" $right
      echo "left" $left
      
      if [[ $right -lt $left ]];
        then
          merged_block=(${merged_block[*]} $right $left)
          right_index=$((right_index+1))
        else
          merged_block=(${merged_block[*]} $left $right)
          left_index=$((left_index+1))
      fi            

    done
}


numbers=(1 2 3 9 5 6)

numbers_length=${#numbers[*]}

block_size=1

while [ $block_size -lt $numbers_length ]
do

  merged_block_size=$((2*block_size))

  left_block_start=0
  right_block_start=$block_size
  
# while using slices based on count taken, final index is not needed
# final index is computed by formula: start_index + block_size - 1, e.g. 0 + 1 -1
# more over final index of right (second) block must corrected when input array is odd
# i.e. right_block_end = min(numbers_length, right_block_start + block_size -1)
# but Bash slices works fine with overindexing
    
  while [ $right_block_start -le $numbers_length ]
  do
    
    left_block=${numbers[*]:left_block_start:block_size}
    right_block=${numbers[*]:right_block_start:block_size}
  
    echo "lb" ${left_block[*]}
    echo "rb" ${right_block[*]}
  
    merge
        
    numbers=("${numbers[*]:0:left_block_start}" "${merged_block[*]:0:merged_block_size}" "${numbers[*]:((right_block_start+block_size)):numbers_length}")
    number=("$numbers")
        
    echo "merged block" ${merged_block[*]}
    echo "numbers" ${numbers[*]}    
        
    left_block_start=$((left_block_start+merged_block_size))
    right_block_start=$((right_block_start+merged_block_size))
    
  done  
  
  block_size=$((block_size*2))
done

echo ${numbers[*]}
