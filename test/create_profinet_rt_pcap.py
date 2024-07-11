from scapy.all import *
import struct

# Function to create a Profinet RT packet
def create_profinet_rt_packet(src_mac, dst_mac, frame_id, cycle_counter, data_status, transfer_status, user_data):
    eth = Ether(src=src_mac, dst=dst_mac, type=0x8892)  # Set EtherType to 0x8892 for Profinet
    profinet_payload = (
        struct.pack('!H', frame_id) + 
        struct.pack('!H', cycle_counter) + 
        struct.pack('!B', data_status) + 
        struct.pack('!B', transfer_status) + 
        user_data
    )
    packet = eth / Raw(load=profinet_payload)
    return packet

# Define details for the Profinet RT packets
packet1 = create_profinet_rt_packet(
    src_mac="00:11:22:33:44:55",
    dst_mac="FF:FF:FF:FF:FF:FF",
    frame_id=0xC000,          # Example Frame ID for Profinet RT
    cycle_counter=1,          # Example Cycle Counter
    data_status=0x80,         # Example Data Status (Good)
    transfer_status=0x01,     # Example Transfer Status (Running)
    user_data=b'\xDE\xAD\xBE\xEF'  # Example User Data
)

packet2 = create_profinet_rt_packet(
    src_mac="00:11:22:33:44:56",
    dst_mac="FF:FF:FF:FF:FF:FE",
    frame_id=0xC001,          # Example Frame ID for Profinet RT
    cycle_counter=2,          # Example Cycle Counter
    data_status=0x80,         # Example Data Status (Good)
    transfer_status=0x01,     # Example Transfer Status (Running)
    user_data=b'\xFE\xED\xFA\xCE'  # Example User Data
)

# Save the Profinet RT packets to a pcap file
pcap_filename = "profinet_rt_packets.pcap"
wrpcap(pcap_filename, [packet1, packet2])

print(f"Profinet RT PCAP file saved as {pcap_filename}")
