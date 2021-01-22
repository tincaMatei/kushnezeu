window.onload = function() {
    // Erase login form if already logged in
    if(get_cookie("session_id") != null) {
        let form = document.getElementById("login");
        form.parentNode.removeChild(form);
        document.getElementById("error").innerText = "You are already logged in";
    }
};

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
                set_cookie("session_id", response.session_id);
                window.location.href = "index.html";
                window.location.replace("index.html");
            } else {
                document.getElementById("error").innerText = response.error_msg;
            }
        }
    };

    xhttp.open("POST", "/api/login", true);
    xhttp.send(query_param);
}
