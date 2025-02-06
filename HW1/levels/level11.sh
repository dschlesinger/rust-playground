ssh -p 2220 bandit11@bandit.labs.overthewire.org

level11_pass="dtR173fZKb0RRsDFSGsg2RWnpNVj3qRr"

# ROT13
cat data.txt | tr 'A-Za-z' 'N-ZA-Mn-za-m'
# 7x16WNeHIi5YkIhWsfFIqoognUTyj9Q4

# N+5 = 5+11 = 16
cat /etc/motd | sed -n '16p'
# 

# Get Line
grep "bandit11" /etc/passwd
# bandit11:x:11011:11011:bandit level 11:/home/bandit11:/bin/bash