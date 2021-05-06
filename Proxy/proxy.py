import os
import threading
import time
import traceback

from subprocess import *
from typing import *
from ruamel.yaml import *
from wakeonlan import send_magic_packet
from socketserver import BaseRequestHandler, TCPServer


################################
#          Variables           #
################################


yaml = YAML()

# get the paths of all files and folders
script_path = os.path.dirname(__file__)
output_path = script_path + '/outputs/'
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

# network variables
HOST = config_data['host_ip']
PORT = config_data['host_PORT']
ADDR = HOST, PORT
high_power_mac_address = config_data['high_power_mac_address']

# loop variables
start_high_power_loop = True


################################
#      Classes/Functions       #
################################


class MyTCPHandler(BaseRequestHandler):
    def handle(self):
        # get th output file name
        output_file_name = str(self.request.recv(1024).strip(), 'utf-8') + '.txt'
        
        # make sure a output file is there to delete its contents
        with open(output_path + output_file_name,"a") as output_file:
            output_file.write('You should not see this.')
            output_file.close()
        
        # delete the output file
        os.remove(output_path + output_file_name)
        
        while True:
            output = str(self.request.recv(1024).strip(), 'utf-8')
            # close the connection if the client closes its
            if not output:
                break
            
            # write the outputs to the output file
            with open(output_path + output_file_name,"a") as output_file:
                output_file.write(output + '\n')
                output_file.close()
        


class MCServer:

    def __init__(self, server_id, output_file_name) -> None:
        self.server_id = server_id
        self.output_file_name = output_file_name
        self.server = 0
        
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

    def outputHandler(self) -> None:
        player_count = 0
        
        # make sure a output file is there to delete its contents
        with open(output_path + self.output_file_name,"a") as output_file:
            output_file.write('You should not see this.')
            output_file.close()
        
        # delete the output file
        os.remove(output_path + self.output_file_name)

        # wait one second till the subprocess is started
        time.sleep(1)
        # read output of the server
        try:
            txt = self.server.stdout
            for line in txt:
                # remove any trailing characters in the output
                output = line.rstrip()
                output = str(output, 'utf-8')

                # write the outputs to the output file
                with open(output_path + self.output_file_name,"a") as output_file:
                    output_file.write(output + '\n')
                    output_file.close()
              
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
# end of MCServer Class #
#########################


def startHighPower() -> None:
    while start_high_power_loop:
        while start_high_power_loop:
            # check every 16 seconds if any player is online
            time.sleep(16)
            
            player_active = server_data['server_1']['player_active']
            
            # if a player gets detected, start the High Power server and wait 5 mins till the next check cycle
            if player_active:
                send_magic_packet(high_power_mac_address)
                print('k')
                break
        
        time.sleep(900)
        

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

    output_file_name = server_name + '.txt'

    mcserver = MCServer(server_id, output_file_name)
    
    # start threads for each function
    threading.Thread(target=mcserver.start).start()
    threading.Thread(target=mcserver.outputHandler).start()
    threading.Thread(target=mcserver.inputHandler).start()

threading.Thread(target=startHighPower).start()

# create a TCP socketserver
with TCPServer((HOST, PORT), MyTCPHandler) as server:
        server.serve_forever()