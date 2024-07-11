#!/bin/bash

# Ensure the script is run with root privileges
if [[ "$EUID" -ne 0 ]]; then
  echo "Please run as root"
  exit 1
fi

# Load environment variables from .env file
if [[ -f ".env" ]]; then
  export $(cat .env | xargs)
else
  echo ".env file not found. Please create a .env file with the INTERFACE variable."
  exit 1
fi

# Check if the INTERFACE variable is set
if [[ -z "$INTERFACE" ]]; then
  echo "INTERFACE variable not set in .env file."
  exit 1
fi

# Generate Profinet RT packets pcap using Python script
echo "Generating Profinet RT packets..."
python3 create_profinet_rt_pcap.py

# Check if the pcap file was created
if [[ ! -f "profinet_rt_packets.pcap" ]]; then
  echo "Failed to create Profinet RT pcap file."
  exit 1
fi

# Replay the pcap file on the network interface
echo "Replaying Profinet RT packets on interface $INTERFACE..."
tcpreplay --intf1=$INTERFACE profinet_rt_packets.pcap

echo "Replay completed."
