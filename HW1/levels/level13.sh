ssh -p 2220 bandit13@bandit.labs.overthewire.org

level13_pass="FO5dwFsc0cbaIiH0h8J2eUks2vdTDwAn"

ssh -i sshkey.private bandit12@localhost -p 2220

cat /etc/bandit_pass/bandit14

# MU4VWeTyJk8ROof1qqmcBPaLh7lDCPvS

# N+5 = 5+13 = 18
cat /etc/motd | sed -n '18p'
#

# Get Line
grep "bandit13" /etc/passwd
# bandit13:x:11013:11013:bandit level 13:/home/bandit13:/bin/bash