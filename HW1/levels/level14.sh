ssh -p 2220 bandit14@bandit.labs.overthewire.org

level14_pass="MU4VWeTyJk8ROof1qqmcBPaLh7lDCPvS"

echo $level14_pass | nc localhost 30000

# 8xCjnmgoKbGLhHFAZlGE5Tmu4M2tKJQo

# N+5 = 5+14 = 19
cat /etc/motd | sed -n '19p'
# If you find any problems, please report them to the #wargames channel on

# Get Line
grep "bandit14" /etc/passwd
# bandit14:x:11014:11014:bandit level 14:/home/bandit14:/bin/bash