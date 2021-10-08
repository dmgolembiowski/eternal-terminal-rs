#!/bin/bash

# Ask user to acknowledge that they've read this and
# have installed their OS's respective dependencies
echo "Please acknowledge that you have already installed \
    your Operating System's required dependencies. If not \
    please visit: (https://github.com/MisterTea/EternalTerminal#building-from-source)"

read -p "Continue (y/n)?" choice
case "$choice" in
  y|Y ) echo "Proceeding with setup...";;
  * ) echo "Exiting..." && exit 0;;
esac

# Retrieve EternalTerminal's C++ library for binding
git submodule update --init --recursive &
wait $!

installation_path=
echo "The build process varies by operating system. The following options are supported."
echo "   (1) Debian/Ubuntu/Kali/Linux-Mint"
echo "   (2) CentOS-7/8"
echo "   (3) Windows/Docker"
echo "   (4) MacOS"
echo "   (5) Other"
read -p "Please select the appropriate option: " os_choice
case "$os_choice" in
  "3") echo "Please see (https://github.com/MisterTea/EternalTerminal#windows)" && exit 0;;
  "5") echo "Sorry, but this script's capabilities are limited. Please try any community suggestions at (https://github.com/MisterTea/EternalTerminal)" && exit 0;;
  "1"|"2"|"4" ) export installation_path="$os_choice";;
  * ) echo "Sorry, but I didn't script this yet. Please see the EternalTerminal official repository for build instructions." && exit 0;;
esac

pushd src/EternalTerminal

if [ -d "./build" ]; then rm -rf ./build/* ; fi
mkdir -p ./build
pushd ./build

if [[ "$installation_path" == "4" ]]; then
    if [[ $(uname -a | grep arm) ]]; then export VCPKG_FORCE_SYSTEM_BINARIES=1; fi
    cmake ../ \
    && make && sudo make install
fi
if [[ "$installation_path" == "1" ]]; then 
    if [[ $(uname -a | grep arm) ]]; then export VCPKG_FORCE_SYSTEM_BINARIES=1; fi
    cmake ../ \
    && make \
    && wait $! \
    && sudo make install && sudo cp ../etc/et.cfg /etc/
fi
if [[ "$installation_path" == "2" ]]; then
    scl enable devtoolset-8 'cmake3 ../' \
    && scl enable devtoolset-8 'make && sudo make install' \
    && sudo cp ../systemctl/et.service /etc/systemd/system/ \
    && sudo cp ../etc/et.cfg /etc/ \
    && export et_loc="$(which etserver)" || ( echo "Could not find \`etserver\` on \$PATH. Please consult the repository's documentation on github." && popd && exit 1 )
    sudo sed -ie "s|ExecStart=[^[:space:]]*[[:space:]]|ExecStart=$(which etserver) |" /etc/systemd/system/et.service \
    && sudo systemctl daemon-reload \
    && sudo systemctl enable --now et.service
fi

echo "Exiting successfully."
popd
