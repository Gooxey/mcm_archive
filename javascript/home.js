// var servers = [
//     {
//         "name": "Server 1",
//         "version": "1.7.10",
//         "type": "Forge",
//         "players": "N/A",
//         "status": "Offline"
//     },
//     {
//         "name": "Server 2",
//         "version": "1.8.9",
//         "type": "Fabric",
//         "players": "2/10",
//         "status": "Online"
//     }
// ];

// replace '__server_id_here__' with the actual server id like 'Server_1'
// var server_html = '<div class="server_tab background_3">\
// <nav>\
//     <ul class="server_tabs noselect">\
//         <li>\
//             <img src="images/list-bullets.svg" id="__server_id_here___panel" onclick="change_server_tab(this.id, "__server_id_here__")">\
//         </li>\
//         <li>\
//             <img src="images/terminal.svg" id="__server_id_here___terminal" onclick="change_server_tab(this.id, "__server_id_here__")">\
//         </li>\
//     </ul>\
// </nav>\
// <div id="__server_id_here___content" class="server_content"></div>\
// \
// <div id="__server_id_here___panel_desc" class="hidden_desc noselect">\
//     <div class="server_active noselect">\
//         <h1>Active</h1>\
//         <label class="switch">\
//             <input type="checkbox">\
//             <span class="slider round"></span>\
//         </label>\
//     </div>\
//     <div class="server_info noselect">\
//         <table>\
//             <tr>\
//                 <th>\
//                     <ul>\
//                         <li><h1>Version</h1></li>\
//                         <li><h1>Type</h1></li>\
//                         <li><h1>Players</h1></li>\
//                         <li><h1>Status</h1></li>\
//                     </ul>\
//                 </th>\
//                 <th>\
//                     <ul>\
//                         <li><h1>1.7.10</h1></li>\
//                         <li><h1>Forge</h1></li>\
//                         <li><h1>N/A</h1></li>\
//                         <li><h1 class="red">Offline</h1></li>\
//                     </ul>\
//                 </th>\
//             </tr>\
//         </table>\
//     </div>\
// </div>\
// <div id="__server_id_here___terminal_desc" class="hidden_desc">\
//     <ul id="__server_id_here___log" class="log background_2 "></ul>\
//     <form class="input background_2" onsubmit="return send_input("__server_id_here__")">\
//         <label for="__server_id_here___input_line">></label>\
//         <input id="__server_id_here___input_line" class="background_2" autocomplete="off">\
//     </form>\
//     <script src="https://code.jquery.com/jquery-1.11.1.js"></script>\
//     <script>\
//         var element = document.getElementById("__server_id_here___log")\
//         \
//         var lines = ["hello", "sjdfosd"]\
//         \
//         lines.forEach(line => {\
//             $("#__server_id_here___log").append($("<li>").text(line));\
//             element.scrollTop = element.scrollHeight - element.clientHeight;\
//         });\
//     </script>\
// </div>\
// </div>';
var server_html =
`<div class="server_tab background_3">\
    <nav>\
        <ul class="server_tabs noselect">\
            <li>\
                <img src="images/list-bullets.svg" id="__server_id_here___panel" onclick="change_server_tab(this.id, '_panel')">\
            </li>\
            <li>\
                <img src="images/terminal.svg" id="__server_id_here___terminal" onclick="change_server_tab(this.id, '_terminal')">\
            </li>\
        </ul>\
    </nav>\
    <div id="__server_id_here___content" class="server_content"></div>\
    \
    <div id="__server_id_here___panel_desc" class="hidden_desc noselect">\
        <div class="server_active noselect">\
            <h1>Active</h1>\
            <label class="switch">\
                <input type="checkbox">\
                <span class="slider round"></span>\
            </label>\
        </div>\
        <div class="server_info noselect">\
            <table>\
                <tr>\
                    <th>\
                        <ul>\
                            <li><h1>Version</h1></li>\
                            <li><h1>Type</h1></li>\
                            <li><h1>Players</h1></li>\
                            <li><h1>Status</h1></li>\
                        </ul>\
                    </th>\
                    <th>\
                        <ul>\
                            <li><h1>1.7.10</h1></li>\
                            <li><h1>Forge</h1></li>\
                            <li><h1>N/A</h1></li>\
                            <li><h1 class="red">Offline</h1></li>\
                        </ul>\
                    </th>\
                </tr>\
            </table>\
        </div>\
    </div>\
    <div id="__server_id_here___terminal_desc" class="hidden_desc">\
        <ul id="__server_id_here___log" class="log background_2 "></ul>\
        <form class="input background_2" onsubmit="return send_input('__server_id_here__')">\
            <label for="__server_id_here___input_line">></label>\
            <input id="__server_id_here___input_line" class="background_2" autocomplete="off">\
        </form>\
        <script src="https://code.jquery.com/jquery-1.11.1.js"></script>\
        <script>\
            var element = document.getElementById("__server_id_here___log")\
            \
            var lines = ["hello", "sjdfosd"]\
            \
            lines.forEach(line => {\
                $("#__server_id_here___log").append($("<li>").text(line));\
                element.scrollTop = element.scrollHeight - element.clientHeight;\
            });\
        </script>\
    </div>\
</div>`;

var servers = [
    {
        "name": "Server 1",
        "version": "1.7.10",
        "type": "Forge",
        "players": "N/A",
        "status": "Offline"
    },
    {
        "name": "Server 2",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 3",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 4",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 5",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 6",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 7",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 8",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 9",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 10",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 11",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 12",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 13",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 14",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 15",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 16",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 17",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 18",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 19",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 20",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 21",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 22",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 23",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 24",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 25",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 26",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 27",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 28",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 29",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 30",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 31",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 32",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 33",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 34",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 35",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 36",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 37",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 38",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    },
    {
        "name": "Server 39",
        "version": "1.8.9",
        "type": "Fabric",
        "players": "2/10",
        "status": "Online"
    }
];

function init_home_tab() {
    servers.forEach(addServersToTable);
    servers.forEach(prepareTab);
}


function addServersToTable(server) {
    var table = document.getElementById("panel_table_body");
    var row = table.insertRow()

    row.insertCell().innerHTML = server["name"];
    row.insertCell().innerHTML = server["version"];
    row.insertCell().innerHTML = server["type"];
    row.insertCell().innerHTML = server["players"];
    var status = row.insertCell();
    status.innerHTML = server["status"];
    if (server["status"] === "Offline") {
        status.className = "red";
    }
    else {
        status.className = "green";
    }

    row.setAttribute("onclick", "openTab(this.id);");
    row.id = server["name"].replace(/\s/g, "_");
}
function prepareTab(server) {
    var div = document.createElement("div");
    div.className = "hidden_desc";
    div.id = server["name"].replace(/\s/g, "_") + "_desc";

    div.innerHTML = server_html.replace(new RegExp("__server_id_here__", "g"), server["name"].replace(/\s/g, "_"));

    document.getElementById("home_tab").appendChild(div)
    change_server_tab(server["name"].replace(/\s/g, "_") + "_panel", "_panel")
}

function openTab(id) {
    var ul = document.getElementById("home_tab_tabs");
    var addelement = true;
    [].forEach.call(ul.getElementsByTagName("li"), function(element) {
        if (element.id === id) {
            addelement = false;
        }
    });
    if (addelement) {
        var li = document.createElement("li");
        li.id = id;
        li.className = "noselect background_2";
        var id_text = document.createElement("span");
        id_text.innerHTML = id;
        id_text.setAttribute("onclick", "change_home_tab(this.parentElement.id);");
        li.appendChild(id_text);
        xbutton = document.createElement("div");
        xbutton.appendChild(document.createTextNode("x"));
        xbutton.className = "xbutton";
        xbutton.id = id.replace(/\s/g, "_") + "_xbutton";
        xbutton.setAttribute("onClick", "closeTab(this.id);");
        li.appendChild(xbutton);
        ul.appendChild(li);
    }
    change_home_tab(id);
}

function closeTab(id) {
    var id = id.replace("_xbutton", "");
    var ul = document.getElementById("home_tab_tabs");
    [].forEach.call(ul.getElementsByTagName("li"), function(element) {
        if (element.id === id) {
            element.remove();
        }
    });
    change_home_tab("panel");
}

function change_home_tab(id) {
    document.getElementById("home_main_content").innerHTML=document.getElementById(id+"_desc").innerHTML;
    var tabs = document.getElementById("home_tab_tabs").getElementsByTagName("li");
    [].forEach.call(tabs, function(element) {
        element.className = "background_2 noselect";
    });
    document.getElementById(id).className = "background_3 noselect";

    if (id === "panel") {
        init_home_tab();
    }
}