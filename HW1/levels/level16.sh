ssh -p 2220 bandit16@bandit.labs.overthewire.org

level16_pass="kSkvUpMQ7lBYyCM4GBPvCvT1BfWRy0Dx"

ports=$(nc -z localhost 31000-32000 2>&1 | grep "succeeded!" | grep -oE "[0-9]{5,5}")

for port in $ports; do
    res=$(echo "$level15_pass" | ncat --ssl localhost $port 2>/dev/null)

    if echo "$res" | grep -q "Correct!"; then
        echo "Port: $port"
        echo "$res"
    fi
done

# Key in level16.key

# N+5 = 5+16 = 21
cat /etc/motd | sed -n '21p'
#

# Get Line
grep "bandit16" /etc/passwd
# bandit16:x:11016:11016:bandit level 16:/home/bandit16:/bin/bash