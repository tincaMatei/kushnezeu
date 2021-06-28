function load_group_list() {
    let session_id = get_cookie("session_id");
    if(session_id != null) {
        let xhttp = new XMLHttpRequest;

        xhttp.onreadystatechange = function() {
            if(this.readyState == 4 && this.status == 200) {
                let response = JSON.parse(this.responseText);
                if(response.error == false) {
                    for(let i = 0; i < response.groups.length; ++i) {
                        let obj_link = document.createElement("a");
                        obj_link.href = "/" + response.groups[i] + "/home";
                        obj_link.innerText = response.groups[i];
                        obj_link.className = "sidebar-button";

                        document.getElementById("group-list").appendChild(obj_link);
                    }
                }
            }
        };

        let query_param = "session_id=" + get_cookie("session_id");

        xhttp.open("POST", "/api/list-groups", true);
        xhttp.send(query_param);
    }
}

function load_home() {
    account_div();

    load_group_list();

    let xhttp2 = new XMLHttpRequest;
    xhttp2.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
            let response = JSON.parse(this.responseText);
            if(response.error == false) {
                let conv = new showdown.Converter()
                document.getElementById("content").innerHTML = conv.makeHtml(response.content);
            } else {
                document.getElementById("content").innerHTML = response.error_msg;
            }
        }
    }

    xhttp2.open("POST", "/api/content/home/home/read", true);
    xhttp2.send(null);
}

function static_load_home() {
    account_div();
    load_group_list();
}

// Load the login page
function load_login_page() {
    hide_login_form();
    load_home();
}

function load_content_page() {
    account_div();
    load_group_list();
    load_content();
}

