ssh -p 2220 bandit20@bandit.labs.overthewire.org

level20_pass="0qXahG8ZjOVMN9Ghs7iOWsCfZyXOUbYO"

# Terminal 1
./suconnect 12345

# Terminal 2
nc -l -p 12345

# EeoULMCra2q0dSkYj561DX7s1CpBuOBt

# N+5 = 5+20 = 25
cat /etc/motd | sed -n '25p'
# If you are playing "somegame", then:

# Get Line
grep "bandit20" /etc/passwd
# bandit20:x:11020:11020:bandit level 20:/home/bandit20:/bin/bash