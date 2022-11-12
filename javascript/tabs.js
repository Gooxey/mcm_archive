function init_tabs() {
    change_tab("home");
    // change_tab("settings");
    change_home_tab("panel");
}
window.onload = init_tabs;

function change_tab(id) {
    document.getElementById("page_content").innerHTML=document.getElementById(id+"_desc").innerHTML;
    document.getElementById("home").className="notselected";
    document.getElementById("settings").className="notselected";
    document.getElementById(id).className="selected";

    if (id=="home") {
        change_home_tab("panel");
    }
}