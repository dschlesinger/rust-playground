ssh -p 2220 bandit19@bandit.labs.overthewire.org

level19_pass="cGWpMaKXVwDUNgPAVJbWYuGHVn9zl3j8"

./bandit20-do cat /etc/bandit_pass/bandit20

# 0qXahG8ZjOVMN9Ghs7iOWsCfZyXOUbYO

# N+5 = 5+19 = 24
cat /etc/motd | sed -n '24p'
# This machine might hold several wargames.

# Get Line
grep "bandit19" /etc/passwd
# bandit19:x:11019:11019:bandit level 19:/home/bandit19:/bin/bash