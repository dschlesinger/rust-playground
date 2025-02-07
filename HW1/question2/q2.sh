wget https://raw.githubusercontent.com/cs109/2015/refs/heads/master/Lectures/Lecture15b/sparklect/shakes/kinglear.txt 2>/dev/null

cat kinglear.txt | tr -s " " | tr " " "\n" | sort | uniq -ic | sort -nr | head -n 2 | tail -n 1