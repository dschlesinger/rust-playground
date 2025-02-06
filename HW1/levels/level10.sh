ssh -p 2220 bandit10@bandit.labs.overthewire.org

level10_pass="FGUW5ilLVJrxX9kMYMmlN4MgbpfMiqey"

base64 -d data.txt
# dtR173fZKb0RRsDFSGsg2RWnpNVj3qRr

# N+5 = 5+10 = 15
cat /etc/motd | sed -n '15p'
#  

# Get Line
grep "bandit10" /etc/passwd
# bandit10:x:11010:11010:bandit level 10:/home/bandit10:/bin/bash