# MCMange
A project aiming to make the creation and management of [Minecraft servers](https://www.minecraft.net) as easy as possible.

## Disclaimer
Almost everything mentioned here is currently not possible since this project is at the start of its development.

## Description

This project consists of three applications and one library used across all of them:

#

### The Proxy
The source code for this application can be found here: https://github.com/Gooxey/mcm_proxy.git

This is the hearth of this project. It is the only application required to be run 24/7 because this is the place where the other two applications connect to or get started from. Therefore all data will be stored here.

#

### The Runner
The source code for this application can be found here: https://github.com/Gooxey/mcm_runner.git

It is optional to use this application because it is only necessary if you do not like to run Minecraft servers all the time or if you have a lot of servers that cannot run on a single computer.

#

### The Client
The source code for this application can be found here: https://github.com/Gooxey/mcm_client.git

To manage all of your Minecraft servers, you can either use this application or the website hosted by the proxy. If both of them are up-to-date, there will be no difference, so it really is a question of personal preference.

#

### The MISC library
The source code for this library can be found here: https://github.com/Gooxey/mcm_misc.git

This library contains functions and structs used by applications in the MCManage Network.

---

For more information about each application, visit their respective repositories.

## License
[GNU General Public License v3.0](https://choosealicense.com/licenses/gpl-3.0/)