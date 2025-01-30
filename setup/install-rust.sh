# 1. Run in terminal -> chmod +x ./install-rust.sh

# 2.
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. Then kill and restart the kerminal then verify installation
bash --login -c "clear && echo 'Installation Successful' && rustc --version && cargo --version"