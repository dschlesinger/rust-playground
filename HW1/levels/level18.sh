ssh -p 2220 bandit18@bandit.labs.overthewire.org "cat readme"

level18_pass="x2gLTTjFwMOhQ8oWNbMN362QKxfRqGlO"

# cGWpMaKXVwDUNgPAVJbWYuGHVn9zl3j8

# N+5 = 5+18 = 23
cat /etc/motd | sed -n '23p'
# 

# Get Line
grep "bandit18" /etc/passwd
# bandit18:x:11018:11018:bandit level 18:/home/bandit18:/bin/bash