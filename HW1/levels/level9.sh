ssh -p 2220 bandit9@bandit.labs.overthewire.org

level9_pass="4CKMh1JI91bUIZZPXDqGanal4xvAg0JM"

strings data.txt | grep -E "={2,}"
# FGUW5ilLVJrxX9kMYMmlN4MgbpfMiqey

# N+5 = 5+9 = 14
cat /etc/motd | sed -n '14p'
#  www. `---` ver     '---' he       '---" ire.org

# Get Line
grep "bandit9" /etc/passwd
# bandit9:x:11009:11009:bandit level 9:/home/bandit9:/bin/bash