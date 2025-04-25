// I don't feel like setting up frontend infra typescript, so... http://vanilla-js.com/

async function init() {
    let data = await fetch("./scrape-viewer.json")
    console.log("response", data)
    console.log("body", data.body)
}

document.addEventListener("load", init)

function update_template(data) {
    document.getElementById()
}