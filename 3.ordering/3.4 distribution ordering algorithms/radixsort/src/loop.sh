for i in {0..500};
do
  eval "cargo test";
  if [[ $? -ne 0 ]];
    then break;    
  fi
  
done

echo $i
