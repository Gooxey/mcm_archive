function change_server_tab(id, element) {
    server = id.replace(element, "");
    document.getElementById(server + "_content").innerHTML=document.getElementById(id+"_desc").innerHTML;
}

function send_input(server) {
    input_line = document.getElementById(server + "_input_line");
    input = input_line.value;
    input_line.value = "";

    log = document.getElementById(server + "_log");
    $("#" + server + "_log").append($('<li>').text(input));
    log.scrollTop = log.scrollHeight - log.clientHeight;

    console.log(input);

    return false;
}