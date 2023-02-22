import socket


TCP_IP = '127.0.0.1'
TCP_PORT = 8080
BUFFER_SIZE = 1024

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect((TCP_IP, TCP_PORT))

s.send(b"select * from table;")
s.send(b"\n\n\n\n")

data = s.recv(BUFFER_SIZE)
print(f"Received: {data}")
s.close()
