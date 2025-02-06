ssh -p 2220 bandit8@bandit.labs.overthewire.org

level8_pass="dfwvzFQi4mU0wfNbFOe9RoWskMLg7eEc"

ls

cat data.txt | sort | uniq -u
# 4CKMh1JI91bUIZZPXDqGanal4xvAg0JM

# N+5 = 5+8 = 13
cat /etc/motd | sed -n '13p'
#     \   \ .'        ;   |.'       \   \ ;

# Get Line
grep "bandit8" /etc/passwd
# bandit8:x:11008:11008:bandit level 8:/home/bandit8:/bin/bash