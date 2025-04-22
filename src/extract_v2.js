(async () => {
    // utils
    function timeout(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    function only_one(arr) {
        if (!Array.isArray(arr)) {
            arr = [...arr];
        }
        if (arr.length !== 1) {
            throw new Error("bad array " + arr.length)
        }
        return arr[0]
    }

    // the whole page is re-hydrated every pagination action, so must requery
    function pagination_parent() {
        let res = [...document.getElementsByClassName("pagination-view")];
        return only_one(res)
    }

    // what page are we on?
    function pagination_current() {
        let pagination = pagination_parent();
        let current_elem = only_one(pagination.querySelectorAll("a:has(span[class='is-current'])"));
        let key = current_elem.attributes["data-track"].value;
        // console.log(`current page key ${key}`)
        let page = parseInt(key.replaceAll("pagination", "").replaceAll("Click", ""));
        console.log(`current page key ${key} num ${page}`)
        return page
    }

    // start at page 1
    if (pagination_current() !== 1) {
        let sleep = 2000;
        let first_page = only_one(pagination_parent().querySelectorAll("a[data-track='pagination1Click']"))
        console.log(`advance to first page, sleep ${sleep}...`)
        first_page.click()
        await timeout(sleep)
    } else {
        console.log("already on first page")
    }

    let imgs = [];
    while (true) {
        let sleep = 2000;
        let current_page = pagination_current();
        console.log(`++SCRAPE - page ${current_page} scroll to bottom, sleep ${sleep}`)
        window.scrollBy(0, 99999);
        await timeout(sleep);

        let counter = 0;
        document.querySelectorAll(".photo-list-photo-container > img").forEach((e) => {
            console.log("img " + e.src);
            imgs.push(e.src)
            counter += 1;
        });
        console.log(`found ${counter} images, total ${imgs.length}`)

        let next_page = [...pagination_parent().querySelectorAll("a[data-track='paginationRightClick']")]
        if (next_page.length === 1) {
            next_page[0].click()
            let sleep = 3000;
            console.log(`advance to next page, sleep ${sleep}`)
            await timeout(sleep)
        } else {
            console.log("no more next pages")
            break;
        }
    }
    console.log("== EXTRACTED IMAGES ==", imgs);
})()