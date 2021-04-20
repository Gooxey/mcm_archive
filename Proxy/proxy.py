import socket
import threading
from typing import *



def createSocket(PORT) -> None:
    HOST = '192.168.178.94'

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind((HOST, PORT))
        s.listen()
        conn, addr = s.accept()
        with conn:
            print('Connected by ', addr)
            while True:
                data = conn.recv(2048)
                if not data:
                    break
                data = data.decode('ascii')
                print(f'{PORT}:     {data}')

def connectionNumber() -> int:
    PORT = 25564
    HOST = '192.168.178.94'

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.bind((HOST, PORT))
        s.listen()
        conn, addr = s.accept()
        with conn:
            while True:
                data = conn.recv(384)
                if not data:
                    break
                data = data.decode('ascii')
                data = int(data)
                return data


connections = connectionNumber()

for i in range(connections):
    PORT = 25563 - i
    print(PORT)
    socketThread = threading.Thread(target=createSocket, kwargs={"PORT":PORT})
    socketThread.start()