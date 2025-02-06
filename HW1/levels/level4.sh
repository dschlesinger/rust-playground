ssh -p 2220 bandit4@bandit.labs.overthewire.org

level4_pass="2WmrDFRmJIq3IPxneAaMGhap0pFhF3NJ"

ls

cd inhere

ls

for i in 1 2 3 4 5 6 7 8 9; do
  cat ./-file0$i
done
# 4oQYVPkxZOOEOO5pTW81FB8j8lxXGUQw

# N+5 = 4+5 = 9
cat /etc/motd | sed -n '9p'
#  .   |  ' ' ' : `----'  |  |  \   ;  `      |

# Get Line
grep "bandit4" /etc/passwd
# bandit4:x:11004:11004:bandit level 4:/home/bandit4:/bin/bash