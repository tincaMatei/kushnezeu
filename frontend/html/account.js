// Hide login or logout
window.onload = function() {
    if(get_cookie("session_id") != null) {
        let element = document.getElementById("login-button");
        element.parentNode.removeChild(element);
    } else {
        let element = document.getElementById("logout-button");
        element.parentNode.removeChild(element);
    }
}

function logout() {
    if(get_cookie("session_id") != null) {
        delete_cookie("session_id");
    }
}

