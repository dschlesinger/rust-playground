ssh -p 2220 bandit12@bandit.labs.overthewire.org

level12_pass="7x16WNeHIi5YkIhWsfFIqoognUTyj9Q4"

dir_name=$(mktemp -d)

cp data.txt $dir_name

cd $dir_name

xxd -r data.txt > data.bin

file data.bin
# Is gzip compressed data

mv data.bin data.gz

gunzip data.gz

ls 

cat data

file data

bzip2 -d data

file data.out

mv data.out data.gz

gunzip data.gz

ls 

cat data

file data
# Tar Archive

tar -xf data

ls

file data5.bin
# Tar again?

tar -xf data5.bin

ls
# :(

file data6.bin

bzip2 -d data6.bin

ls 

file data6.bin.out
# Tar again???

tar -xf data6.bin.out

ls 

file data8.bin
# Gzip compressed data

mv data8.bin data8.gz

gunzip data8.gz

ls 

file data8

cat data8

# FO5dwFsc0cbaIiH0h8J2eUks2vdTDwAn

# N+5 = 5+12 = 17
cat /etc/motd | sed -n '17p'
# Welcome to OverTheWire!

# Get Line
grep "bandit12" /etc/passwd
# bandit12:x:11012:11012:bandit level 12:/home/bandit12:/bin/bash