ssh -i HW1/levels/level16.key -p 2220 bandit17@bandit.labs.overthewire.org

cat passwords.new passwords.old | sort | uniq -u

cat passwords.new | grep "x2gLTTjFwMOhQ8oWNbMN362QKxfRqGlO"
# Found so this one
# x2gLTTjFwMOhQ8oWNbMN362QKxfRqGlO

# N+5 = 5+17 = 22
cat /etc/motd | sed -n '22p'
# --[ Playing the games ]--

# Get Line
grep "bandit17" /etc/passwd
# bandit17:x:11017:11017:bandit level 17:/home/bandit17:/bin/bash