from flask import Flask, make_response
from lxml import etree

app = Flask(__name__)

@app.route("/")
@app.route("/index.html")
def index():
    html = open("index.html").read()
    return html

@app.route("/data_measures_wasm_bg.js")
def data_measures_wasm_bg_js():
    r = make_response(open("pkg/data_measures_wasm_bg.js").read())
    r.headers.set('Content-Type', "text/javascript")
    return r

@app.route("/data_measures_wasm.js")
def data_measures_wasm_js():
    r = make_response(open("pkg/data_measures_wasm.js").read())
    r.headers.set('Content-Type', "text/javascript")
    return r

@app.route("/data_measures_wasm_bg.wasm")
def data_measures_wasm_bg_wasm():
    r = make_response(open("pkg/data_measures_wasm_bg.wasm","rb").read())
    r.headers.set('Content-Type', "application/wasm")
    return r

if __name__ == "__main__":
    app.run(debug=True,port=8080)