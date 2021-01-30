function load_home() {
    account_div();
    
    let session_id = get_cookie("session_id");
    if(session_id != null) {
        let xhttp = new XMLHttpRequest;

        xhttp.onreadystatechange = function() {
            if(this.readyState == 4 && this.status == 200) {
                let response = JSON.parse(this.responseText);
                if(response.error == false) {
                    for(let i = 0; i < response.groups.length; ++i) {
                        let obj = document.createElement("li");
                        let obj_link = document.createElement("a");
                        obj_link.href = "/content/" + response.groups[i] + "/home";
                        obj_link.innerText = response.groups[i];
                    
                        obj.appendChild(obj_link);
                        document.getElementById("group-list").appendChild(obj);
                    }
                }
            }
        };

        let query_param = "session_id=" + get_cookie("session_id");

        xhttp.open("POST", "/api/list-groups", true);
        xhttp.send(query_param);
    }
}
