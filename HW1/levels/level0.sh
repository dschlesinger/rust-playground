ssh -p 2220 bandit0@bandit.labs.overthewire.org

level0_pass="bandit0"

ls

cat readme
# ZjLjTmM6FvvyRnrb2rfNWOZOTa6ip5If

# N+5 = 0+5 = 5
cat /etc/motd | sed -n '5p'
# .   /   ;.  \   ;    ;     /    /__./ \ : |

# Get Line
grep "bandit0" /etc/passwd
# bandit0:x:11000:11000:bandit level 0:/home/bandit0:/bin/bash