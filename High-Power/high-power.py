import io
import os
from socket import *
import threading
import time
from subprocess import PIPE, Popen, STDOUT
from ruamel.yaml import YAML


################################
#          Variables           #
################################


script_path = os.path.dirname(__file__)

# load the server-data
yaml = YAML()
server_data_file = open(script_path + '/server.yml', 'r')
server_data = yaml.load(server_data_file)

file_name = script_path + '/server.yml'
server_count = server_data['server_count']


################################
#      Classes/Functions       #
################################


class ComHandler:

    def __init__(self, mc_server, PORT) -> None:
        self.mc_server = mc_server

        HOST = '192.168.178.94'
        ADDR = (HOST, PORT)
        
        # create a socket connection to the proxy server
        self.serversock = socket(AF_INET, SOCK_STREAM)
        self.serversock.connect(ADDR)

        
    def outputHandler(self, server_id, file_name) -> None:
        player_count = 0

        # read output of the process
        for line in io.TextIOWrapper(self.mc_server.stdout, encoding="utf-8"):
            output = line.rstrip()

            # check if anyone is online
            if 'logged' in output:
                player_count += 1
            if 'left' in output:
                player_count -= 1

            # send the outputs to the proxy server
            output = output.encode('ascii')
            self.serversock.sendall(output)

            # change player_active info to their right value
            if player_count > 0:
                server_data[server_id]['player_active'] = True
                
                with open(file_name , 'w') as fp:
                    yaml.dump(server_data, fp)
            else:
                server_data[server_id]['player_active'] = False
                
                with open(file_name , 'w') as fp:
                    yaml.dump(server_data, fp)
    
    def inputHandler(self) -> None:
        pass

    def shutdown(self) -> None:
        time.sleep(960)
        
        while(True):
            time.sleep(1)
            # check if shutdown command is given
            shutdown = server_data['shutdown']
            if shutdown:
                # shutdown the server if command given
                self.mc_server.communicate(b'stop\n')

def starter(server_path, jar_name, max_ram, server_id, PORT) -> None:
    # reset player_active info
    server_data[server_id]['player_active'] = False

    with open(file_name , 'w') as fp:
        yaml.dump(server_data, fp)

    # start a mc_server
    mc_server = Popen(
        ['java', f'-Xmx{max_ram}', '-jar', jar_name],
        cwd = server_path,
        stdin = PIPE,
        stdout = PIPE,
        stderr = STDOUT,
        shell = True
    )
    server = ComHandler(mc_server, PORT)
    
    # start threads for Handling with the servers
    outputHandler = threading.Thread(
        target=server.outputHandler,
        kwargs={
            "server_id": server_id,
            "file_name": file_name
            }
    )
    inputHandler = threading.Thread(target=server.inputHandler)
    shutdown = threading.Thread(target=server.shutdown)
    outputHandler.start()
    inputHandler.start()
    shutdown.start()

def stopServer() -> None:
    server_used = False
    
    #reset shutdown info to normal state
    server_data['shutdown'] = False
                
    with open(file_name , 'w') as fp:
        yaml.dump(server_data, fp)
    

    time.sleep(300)
    
    # check if anyone is online
    while(True):
        while(True):
            time.sleep(1)
            # check every server for online player status
            for i in range(server_count):
                server_id = f'server_{i}'
                player_active = server_data[server_id]['player_active']

                if player_active:
                    server_used = True
                    break
            # if nobody is online wait 15 min
            if server_used is False:
                break
        
        time.sleep(900)
    	
        # check again for any online players
        for i in range(server_count):
                server_id = f'server_{i}'
                player_active = server_data[server_id]['player_active']

                if player_active:
                    server_used = True
                    break
        # if there is still no one online shut down every server and the pc
        if server_used is False:
            break
    
    # shutdown of the servers
    server_data['shutdown'] = True
                
    with open(file_name , 'w') as fp:
        yaml.dump(server_data, fp)
    
    time.sleep(300)
    
    # shutdown of the High-Power-PC
    #os.system("shutdown /s /t 1")


################################
#            Code              #
################################


HOST = '192.168.178.94'
PORT = 25564
ADDR = (HOST, PORT)
            
# create a socket connection to the proxy server
serversock = socket(AF_INET, SOCK_STREAM)
serversock.connect(ADDR)

connections = f'{server_count}'
connections = str.encode(connections)
serversock.sendall(connections)

# start every server with the right attributes
for i in range(server_count):
    server_id = f'server_{i}'
    PORT = 25563 - i
    
    # get the required values for each server from the yml file
    server_name = server_data[server_id]['server_name']
    server_path = script_path + server_data[server_id]['server_path']
    jar_name = server_data[server_id]['jar_name']
    max_ram = server_data[server_id]['max_ram']

    starter(server_path, jar_name, max_ram, server_id, PORT)   # start a server with the checked attributes

stopServer()