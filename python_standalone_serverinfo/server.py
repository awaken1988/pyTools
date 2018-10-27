import os
import traceback
import logging
import socket
import subprocess
from settings import *
from rocket import Rocket
from wsgiref.util import setup_testing_defaults
from wsgiref.simple_server import make_server

if "$$$TITLE$$$" not in SUBSTITUTE_TEXT:
    SUBSTITUTE_TEXT["$$$TITLE$$$"] = socket.gethostname()

def insert_modules(aContent):
    content = ""
    
    for iMod in SRVINFO_MODULES:
        content += "<div class='module_div'>"
        content += "<h2 class='module_div'>" + iMod.get_name() + "</h2>"
        content += iMod.content()
        content += "</div>"

    print("blaaaaa")

    return aContent.replace("$$$MODULE_CONTENT$$$", content)

def substitue_content(aContent):
    for iSubst in SUBSTITUTE_TEXT:
        aContent = aContent.replace(iSubst, SUBSTITUTE_TEXT[iSubst])
    return aContent

#server main procedure
def my_simple_app(environ, start_response):
    status = '200 OK'
    content_type = "text/plain"
   
    ret = ""
    SUBSTITUTE_TEXT["$$$DEBUG_WSGI$$$"] = str(environ)
    
    SUBSTITUTE_TEXT["$$$QUICKINFO$$$"] = ""
    SUBSTITUTE_TEXT["$$$QUICKINFO$$$"] += subprocess.run("date '+%Y-%M-%d__%H:%m'", shell=True,  capture_output=True).stdout.decode('utf-8')
    SUBSTITUTE_TEXT["$$$QUICKINFO$$$"] += " | uptime=" + subprocess.run("uptime -p", shell=True,  capture_output=True).stdout.decode('utf-8')
    SUBSTITUTE_TEXT["$$$QUICKINFO$$$"] += " | users=" + subprocess.run("users", shell=True,  capture_output=True).stdout.decode('utf-8')


    try:
        path_info = environ["PATH_INFO"]
        if path_info in MAP_FILES:
            path_info = MAP_FILES[path_info]
        if path_info in FILES:
            content_type = FILES[path_info]
            path_info = os.path.join(os.getcwd(), path_info.strip("/"))
            f = open(path_info, "r")
            ret = substitue_content(f.read())

            if "$$$MODULE_CONTENT$$$" in ret:   ret = insert_modules(ret)

        else:
            ret += str(environ)
    except:
        traceback.print_exc()

    headers = [('Content-type', content_type)]
    start_response(status, headers)
    return ret

my_demo = { "wsgi_app": my_simple_app }

if __name__ == '__main__':
    log = logging.getLogger('Rocket.Requests')
    log.setLevel(logging.INFO)
    fmt = logging.Formatter('%(levelname)s:%(name)s:%(message)s')
    h = logging.StreamHandler()
    h.setFormatter(fmt)
    log.addHandler(h)

    r = Rocket(interfaces=[('127.0.0.1', 8000), ('::1', 8000)], method='wsgi', app_info=my_demo)
    r.start()