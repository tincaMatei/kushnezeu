function get_cookie(cname) {
    let cookie = document.cookie;
    let tokens = cookie.split(';');

    for(let i = 0; i < tokens.length; ++i) {
        var parts = tokens[i].split('=');
        for(let j = 0; j < parts.length; ++j) {
            parts[j] = parts[j].trim();
        }

        if(parts[0] == cname) {
            return parts[1];
        }
    }
    return null;
}

function set_cookie(cname, value, expiredate=null) {
    let cookie_str = cname + "=" + value + ";";
    if(expiredate != null) {
        cookie_str = cookie_str + "expires=" + expiredate;
    }

    document.cookie = cookie_str;
}

function delete_cookie(cname) {
    document.cookie = cname + "=; expires=Thu, 01 Jan 1970 00:00:00 UTC";
}

