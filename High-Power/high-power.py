import os
import sys
import threading
import time
import traceback

from subprocess import *
from ruamel.yaml import *
from socket import *


################################
#          Variables           #
################################


yaml = YAML()

# get the paths of all files and folders
script_path = os.path.dirname(__file__)
config_data_path = script_path + '/config.yml'
server_data_path = script_path + '/server.yml'

# load config_data for reading
config_data_file = open(config_data_path, 'r')
config_data = yaml.load(config_data_file)

# load server_data for reading
server_data_file = open(server_data_path, 'r')
server_data = yaml.load(server_data_file)

# MC server relevant variables
server_count = server_data['server_count']
server_used = False

# network variables
HOST = config_data['proxy_ip']
PORT = config_data['proxy_PORT']
ADDR = HOST, PORT

# loop variables
shutdown_loop = True


################################
#      Classes/Functions       #
################################


class MCServer:

    def __init__(self, server_id, server_name) -> None:
        self.server_id = server_id
        self.server = 0

        # create a socket connection to the proxy server
        self.serversock = socket(AF_INET, SOCK_STREAM)
        self.serversock.connect(ADDR)
        self.serversock.sendall(bytes(server_name, 'utf-8'))
     
    def start(self) -> None:
        # start a subprocess for the MC server
        self.server = Popen(
            ['java', f'-Xmx{max_ram}', '-jar', jar_name],
            cwd = server_path,
            stdin = PIPE,
            stdout = PIPE,
            stderr = STDOUT,
            shell = True
        )
    
    def stop(self):
        # wait 5 min to give every server and thread a chance to start
        time.sleep(300)
        
        # check every 16 seconds if the server should shut down
        while(True):
            time.sleep(16)
            
            # check if shutdown command is given
            shutdown = server_data['shutdown']
            
            # start the shutdown process if the comand is given
            if shutdown:
                # reset player_active value
                server_data[self.server_id]['player_active'] = False
                with open(server_data_path , 'w') as fp:
                    yaml.dump(server_data, fp)

                # shutdown the MC server
                self.server.communicate(b'stop\n')
                self.serversock.close()
                sys.exit()

    def outputHandler(self) -> None:
        time.sleep(1)
        
        player_count = 0
        # read output of the server
        try:
            txt = self.server.stdout
            for line in txt:
                # remove any trailing characters in the output
                output = line.rstrip()
                output = str(output, 'utf-8')

                # send outputs to proxy
                self.serversock.sendall(bytes(output + '\n', 'utf-8'))
              
                # check if anyone is online and save in the player_count variable
                if 'logged in with entity id' in output:
                    player_count += 1
                if 'left the game' in output:
                    player_count -= 1

                # change the player_active value depending on the player_count value
                if player_count:
                    server_data[self.server_id]['player_active'] = True
                    with open(server_data_path , 'w') as fp:
                        yaml.dump(server_data, fp)
                else:
                    server_data[self.server_id]['player_active'] = False
                    with open(server_data_path , 'w') as fp:
                        yaml.dump(server_data, fp)
        
        except Exception as err:
            print('Error', err, txt)
            print(traceback.format_exc())

    def inputHandler(self):
        # to be done in future
        pass


#########################
# End of MCServer Class #
# shutdown functions    #
#########################


def checkServerState():
    # check every registered server
    for i in range(server_count):
        # create a variable for the meant server
        server_id = f'server_{i}'
        # get the player active value of the meant server
        player_active = server_data[server_id]['player_active']
        
        # if any server is used by a player break the loop
        if player_active:
                server_used = True
                break
        
        # set the server_used value to false if no server is used
        if not player_active:
            server_used = False

def shutdown():
    first_thread_active = False
    
    # reset shutdown info to the standard state
    server_data['shutdown'] = False           
    with open(server_data_path , 'w') as fp:
        yaml.dump(server_data, fp)
    
    # wait 5 min
    time.sleep(300)
    
    while shutdown_loop:
        # check every 16 seconds if anyone is playing on the servers
        while shutdown_loop:
            time.sleep(4)
            
            # only start a new checking-thread if the old one finished
            if not first_thread_active:
                check_server_state_timer = threading.Timer(16, checkServerState)
                check_server_state_timer.start()
            
            # if the servers are not used break out of the loop to try stopping the servers
            if not server_used:
                break
        
            # check if the checking-thread is still active
            first_thread_active =  check_server_state_timer.is_alive()
        
        # start the check_server_state function a second time after 15 min to make sure nobody is online
        second_check_server_state_timer = threading.Timer(900, checkServerState)
        second_check_server_state_timer.start()

        # if the servers are still not used start the shutdown-process
        if not server_used:
            break
    
    # shutdown of the servers
    server_data['shutdown'] = True
    with open(server_data_path , 'w') as fp:
        yaml.dump(server_data, fp)

    # wait 5 min till every server has stopped
    time.sleep(300)

    # shutdown of the High-Power-PC
    os.system("shutdown /s /t 1")

################################
#            Code              #
################################


for i in range(server_count):
    i = str(i)
    server_id = 'server_' + i
    
    # get the required values for each server from the yml file
    server_name = server_data[server_id]['server_name']
    server_path = script_path + server_data[server_id]['server_path']
    jar_name = server_data[server_id]['jar_name']
    max_ram = server_data[server_id]['max_ram']

    # start threads for every funktion
    mcserver = MCServer(server_id, server_name)
    
    time.sleep(1)

    threading.Thread(target=mcserver.start).start()
    threading.Thread(target=mcserver.stop).start()
    threading.Thread(target=mcserver.outputHandler).start()
    threading.Thread(target=mcserver.inputHandler).start()

threading.Thread(target=shutdown).start()