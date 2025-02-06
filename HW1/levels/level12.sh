ssh -p 2220 bandit12@bandit.labs.overthewire.org

level12_pass="7x16WNeHIi5YkIhWsfFIqoognUTyj9Q4"

# Uh password is in tmp/temp, data.txt is not compressed?

# FO5dwFsc0cbaIiH0h8J2eUks2vdTDwAn

# N+5 = 5+12 = 17
cat /etc/motd | sed -n '17p'
# Welcome to OverTheWire!

# Get Line
grep "bandit12" /etc/passwd
# bandit12:x:11012:11012:bandit level 12:/home/bandit12:/bin/bash