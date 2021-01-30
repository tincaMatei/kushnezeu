function load_content() {
    let path_array = window.location.pathname.split('/');
    let session_id = get_cookie("session_id");
    if(path_array.length == 4 && session_id != null) {
        let xhttp = new XMLHttpRequest;
        let query_param = "session_id=" + session_id;

        xhttp.onreadystatechange = function() {
            if(this.readyState == 4 && this.status == 200) {
                let response = JSON.parse(this.responseText);
                console.log(response);
                if(response.error == false) {
                    let conv = new showdown.Converter();
                    document.getElementById("content").innerHTML = conv.makeHtml(response.content);
                    document.getElementById("content-textarea").value = response.content;
                } else {
                    document.getElementById("content").innerHTML = response.error_msg;
                }
            }
        };
        
        let pathname = "/api/content/" + path_array[2] + "/" + path_array[3] + "/read";
        console.log(pathname);
        xhttp.open("POST", pathname, true);
        xhttp.send(query_param);

        let xhttp2 = new XMLHttpRequest;
        xhttp2.onreadystatechange = function() {
            if(this.readyState == 4 && this.status == 200) {
                let response = JSON.parse(this.responseText);
                console.log(response);
                if(response.rights[1] != 'W') {
                    let button = document.getElementById("edit-button");
                    button.parentElement.removeChild(button);
                }
            }
        };

        let pathname2 = "/api/get-rights/" + path_array[2];
        xhttp2.open("POST", pathname2, true);
        xhttp2.send(query_param);
    } else if(session_id == null) {
        window.location.href = "/login";
        window.location.replace("/login");
    } else {
        // Automatic redirect to home
        window.location.href = "/home";
        window.location.replace("/home");
    }
}

function open_writing() {
    document.getElementById("text-editor").style.visibility = "visible";
    document.getElementById("edit-button").disabled = true;
}

function preview_content() {
    document.getElementById("preview-button").disabled = true;
    
    let conv = new showdown.Converter();
    let txt = document.getElementById("content-textarea").value;
    document.getElementById("content").innerHTML = conv.makeHtml(txt);

    document.getElementById("preview-button").disabled = false;
    console.log("TEST");
}

function cancel_editing() {
    console.log("Cancel editing");
    document.getElementById("edit-button").disabled = false;
    document.getElementById("text-editor").style.visibility = "hidden";
}

function submit_content_edit() {
    document.getElementById("save-button").disabled = true;

    let xhttp = new XMLHttpRequest;
    xhttp.onreadystatechange = function() {
        if(this.readyState == 4 && this.status == 200) {
            let response = JSON.parse(this.responseText);
            if(response.error == true) {
                document.getElementById("error-msg").innerText = response.error_msg;
                document.getElementById("save-button").disabled = false;
            } else {
                window.location.reload();
            }
        }
    };

    let query_param = "session_id=" + get_cookie("session_id") 
                    + "&content=" + document.getElementById("content-textarea").value;
    let path_array = window.location.pathname.split('/');
    let path_name = "/api/content/" + path_array[2] + "/" + path_array[3] + "/write";
    xhttp.open("POST", path_name, true);
    xhttp.send(query_param);
}

