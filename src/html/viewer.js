// I don't feel like setting up frontend infra typescript, so... http://vanilla-js.com/

function init() {
    // book_viewer is loaded from JS-ified JSON
    let users = Object.keys(book_viewer.photos);

    load_user_filter(users);
    load_images(users, book_viewer.photos);

    console.log("hi", book_viewer)
}

document.addEventListener("DOMContentLoaded", init)

function load_user_filter(users) {
    const user_tpl = document.getElementById("user-tpl");
    for (const user of users) {
        console.log("adding user " + user)
        let option = user_tpl.cloneNode();
        option.removeAttribute("id");
        option.setAttribute("value", user);
        option.innerHTML = "u " + user;
        user_tpl.parentElement.appendChild(option)
    }
    user_tpl.remove()
}

function load_images(users, user_lookup) {
    const photo_tpl = document.getElementById("photo-single-tpl");
    let total = 0;
    outer: for (const user of users) {
        const photos = user_lookup[user];
        // console.log(`user ${user} has ${photos.length}`)

        for (const photo_src of photos) {
            total += 1;
            if (total > 4000) {
                break outer;
            }

            let photo_div = photo_tpl.cloneNode(true);
            photo_div.removeAttribute("id");

            let img_elem = photo_div.querySelector("#photo-img-tpl");
            img_elem.removeAttribute("id");
            img_elem.setAttribute("src", photo_src.url);

            photo_tpl.parentElement.appendChild(photo_div)
        }
    }
    photo_tpl.remove()
}