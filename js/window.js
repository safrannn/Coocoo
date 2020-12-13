var placeholder_img_src = "https://user-images.githubusercontent.com/20684618/31289519-9ebdbe1a-aae6-11e7-8f82-bf794fdd9d1a.png"


export const add_to_carousel = function (carousel_id, img_name, img_src) {
    var place_holder = document.getElementById(carousel_id + "_ind_place_holder");
    if (typeof (place_holder) != 'undefined' && place_holder != null) {
        place_holder.remove();
        document.getElementById(carousel_id + "_inn_place_holder").remove();
    }

    var carousel_indicator = document.getElementById(carousel_id + "_carousel_indicator");
    var new_carousel_indicator_item = document.createElement("li");
    if (carousel_indicator.childElementCount == 0) {
        new_carousel_indicator_item.setAttribute("class", "active");
        new_carousel_indicator_item.setAttribute("data-target", "#" + carousel_id + "_carousel_indicator");
    } else {
        new_carousel_indicator_item.setAttribute("data-target", "#" + carousel_id + "_carousel_indicator");

    }
    carousel_indicator.appendChild(new_carousel_indicator_item);

    var carousel_inner = document.getElementById(carousel_id + "_carousel_inner");
    var new_inner_item = document.createElement("div");
    if (carousel_inner.childElementCount == 0) {
        new_inner_item.setAttribute("class", "carousel-item active");
    } else {
        new_inner_item.setAttribute("class", "carousel-item");
    }
    var new_inner_image = document.createElement("img");
    new_inner_image.setAttribute("class", "d-block mx-auto mh-100 mw-100");
    new_inner_image.alt = img_name;
    new_inner_image.src = img_src;
    var new_inner_caption = document.createElement("div");
    new_inner_caption.setAttribute("class", "carousel-caption d-none d-md-block");
    var new_inner_caption_h = document.createElement("h5");
    new_inner_caption_h.appendChild(document.createTextNode(img_name));
    new_inner_caption.appendChild(new_inner_caption_h);
    new_inner_item.appendChild(new_inner_caption);
    new_inner_item.appendChild(new_inner_image);
    carousel_inner.appendChild(new_inner_item);
}

function clear_carousel(carousel_id) {
    var carousel_indicator = document.getElementById(carousel_id + "_carousel_indicator");
    while (carousel_indicator.lastElementChild) {
        carousel_indicator.removeChild(carousel_indicator.lastElementChild);
    }
    var carousel_inner = document.getElementById(carousel_id + "_carousel_inner");
    while (carousel_inner.lastElementChild) {
        carousel_inner.removeChild(carousel_inner.lastElementChild);
    }
}

function add_placeholder_to_carousel(carousel_id) {
    var carousel_indicator = document.getElementById(carousel_id + "_carousel_indicator");
    var placeholder_ind = document.createElement("li");
    placeholder_ind.setAttribute("id", carousel_id + "_ind_place_holder");
    placeholder_ind.setAttribute("data-target", "#" + carousel_id + "_carousel");
    placeholder_ind.setAttribute("class", "active");
    carousel_indicator.appendChild(placeholder_ind);

    var carousel_inn = document.getElementById(carousel_id + "_carousel_inner");
    var placeholder_inn = document.createElement("div");
    placeholder_inn.setAttribute("id", carousel_id + "_inn_place_holder");
    placeholder_inn.setAttribute("class", "carousel-item active");
    var placeholder_inn_img = document.createElement("img");
    placeholder_inn_img.setAttribute("class", "d-block mx-auto mh-100 mw-100");
    placeholder_inn_img.setAttribute("src", placeholder_img_src);
    placeholder_inn_img.setAttribute("alt", "place_holder");
    placeholder_inn.appendChild(placeholder_inn_img);
    carousel_inn.appendChild(placeholder_inn);

}
export function reset_carousel(carousel_id) {
    clear_carousel(carousel_id);
    add_placeholder_to_carousel(carousel_id);
}

export const window_onload = async function () {
    document.body.classList.add("loading");
    document.getElementById('download_button').onclick = function () {
        this.href = document.getElementById("imageCanvas_modified").toDataURL();
        this.download = "image.png";
    };

    $(document).ready(function () {
        $('#sidebarCollapse').on('click', function () {
            $('#sidebar').toggleClass('active');
        });

    });

    document.getElementById('reset').onclick = function () {
        reset_carousel("original_images");
        reset_carousel("result_images");
    };
};

export function show_result_images(result_images) {
    reset_carousel("result_images");
    console.log("result images:", result_images);
    for (let [img_name, img_data] of Object.entries(result_images)) {
        add_to_carousel("result_images", img_name, image_to_src(img_data.pixels, img_data.width, img_data.height));
    }
}


