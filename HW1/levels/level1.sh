ssh -p 2220 bandit1@bandit.labs.overthewire.org

level1_pass="ZjLjTmM6FvvyRnrb2rfNWOZOTa6ip5If"

ls

cat ./-
# 263JGJPfgU6LtdEvgfWU1XP5yac29mFx

# N+5 = 1+5 = 6
cat /etc/motd | sed -n '6p'
#   .   ;   /  ` ; .'___,/    ,' .--'.  '   \' .

# Get Line
grep "bandit1" /etc/passwd
# bandit1:x:11001:11001:bandit level 1:/home/bandit1:/bin/bash