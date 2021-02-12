import sys
import time
import socket

addr = "0.0.0.0"

if len(sys.argv) > 1:
  addr = sys.argv[1]

client = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
client.settimeout(1.0)

addr = (addr, 41431)

while True:
  message = input()
  message_b = bytes(message, "utf-8")

  client.sendto(message_b, addr)