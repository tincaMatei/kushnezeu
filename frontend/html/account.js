// Hide the login form if already logged in
function hide_login_form() {
    if(get_cookie("session_id") != null) {
        let form = document.getElementById("login");
        form.parentNode.removeChild(form);
        document.getElementById("error").innerText = "You are already logged in";
    }
}

// Display the account section accordingly
function account_div() {
    if(get_cookie("session_id") != null) {
        let element = document.getElementById("login-button");
        element.parentNode.removeChild(element);
        document.getElementById("username-display").innerText = "Username: " + get_cookie("username");
    } else {
        let element = document.getElementById("logout-button");
        element.parentNode.removeChild(element);
    }
}

// Submit the login form to the backend
function submit_login() {
    let element = document.getElementById("login");
    element.onsubmit = "javascript:void(0)";

    var xhttp = new XMLHttpRequest;
    let query_param = "username=" + document.forms["login"]["username"].value + 
                      "&password=" + document.forms["login"]["password"].value;

    xhttp.onreadystatechange = function() {
        console.log("Made a request: " + query_param);
        if (this.readyState == 4 && this.status == 200) {
            let response = JSON.parse(this.responseText);
            console.log(response.session_id);
            element.onsubmit = submit_login;
            if(response.error == false) {
                let expire_date = new Date(response.expire)
                console.log("Expire date: " + expire_date);
                set_cookie("session_id", response.session_id, expire_date);
                set_cookie("username", document.forms["login"]["username"].value, expire_date);
                window.location.href = "/home";
                window.location.replace("/home");
            } else {
                document.getElementById("error").innerText = response.error_msg;
            }
        }
    };

    xhttp.open("POST", "/api/login", true);
    xhttp.send(query_param);
}

// Logout and delete the session
function submit_logout() {
    console.log("Called logout");
    if(get_cookie("session_id") != null) {
        console.log("Deleting session");
        let xhttp = new XMLHttpRequest;
        let query_param = "session_id=" + get_cookie("session_id");

        xhttp.onreadystatechange = function() {
            if(this.readyState == 4 && this.status == 200) {
                delete_cookie("session_id");
                delete_cookie("username");
                console.log("Erase cookie");
                window.location.href = "/home";
                window.location.replace("/home");
            }
        }

        xhttp.open("POST", "/api/logout", true);
        xhttp.send(query_param);
    }
}

// Refresh the session time and expiration date
function refresh_session() {
    if(get_cookie("session_id") != null) {
        console.log("Refreshing session:" + get_cookie("session_id"));
        let xhttp = new XMLHttpRequest;
        let query_param = "session_id=" + get_cookie("session_id");

        xhttp.onreadystatechange = function() {
            if(this.readyState == 4 && this.status == 200) {
                let response = JSON.parse(this.responseText);
                if(response.expire != null) {
                    let my_id = get_cookie("session_id");
                    set_cookie("session_id", my_id, response.expire);
                }
            }
        }
    }

    xhttp.open("POST", "/api/refresh-session");
    xhttp.send(query_param);
}

