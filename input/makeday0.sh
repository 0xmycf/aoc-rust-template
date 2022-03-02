# adds numbers 1-250 to day0.txt
touch day0.txt
END=250
for ((i=1;i<=END;i++)); do
    echo $i >> day0.txt
done
