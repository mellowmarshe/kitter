/*

Utils

*/

const serialize_form = form => JSON.stringify(
    Array.from(new FormData(form).entries())
    .reduce((m, [key, value]) => Object.assign(m, {
        [key]: value
    }), {})
);

function add_post(post, first) {
    var element = $("#posts");
    var json = post;
    var time = new Date(`${json["timestamp"]}`).toLocaleString();

    var del_html = (json["author_id"] == parseInt(user)) ?
        `
<div class="dropdown-item">
    <a href="#" class="dropdown-item" id="dropdown-delete"
        value="${json['id']}">
        Delete
    </a>
</div>
` : ''

    var el = `<div class="box is-dark  post-${json["id"]}">
                <article class="media">
                    <div class="media-left">
                        <figure class="image is-64x64">
                            <img src="https://i.imgur.com/z6g7ZiE.jpg" alt="Image">
                        </figure>
                    </div>
                    <div class="media-content">
                        <div class="content">
                            <p>
                                <strong>${json["author"]}</strong>
                                    <small>${time}</small>
                                </span>
                                <br>
                                <span style="white-space: pre-wrap;">${json["content"]}</span>
                            </p>
                        </div>
                        <nav class="level is-mobile">
                            <div class="level-left">
                                <a class="level-item" aria-label="reply">
                                    <span class="icon is-small">
                                        <i class="fa fa-reply is-black"></i>
                                    </span>
                                </a>
                                <a class="level-item" aria-label="like">
                                    <span class="icon is-small">
                                        <i class="fa fa-heart is-black"></i>
                                    </span>
                                </a>
                            </div>
                            <div class="level-right">
                                <a class="level-item">
                                    <div class="dropdown dropdown-${json["id"]}">
                                        <div class="dropdown-trigger">
                                            <button class="button" aria-haspopup="true" aria-controls="dropdown-menu2">
                                                <span class="icon is-small">
                                                    <i class="fa fa-ellipsis-h" aria-hidden="true"></i>
                                                </span>
                                            </button>
                                        </div>
                                        <div class="dropdown-menu" id="dropdown-menu2" role="menu">
                                            <div class="dropdown-content">
                                                <div class="dropdown-item">
                                                    <a href="#" class="dropdown-item">
                                                        Report
                                                    </a>
                                                </div>
                                                ${del_html}
                                            </div>
                                        </div>
                                    </div>
                                </a>
                            </div>
                        </nav>
                    </div>
                </article>
            </div>  
`

    if (first) {
        element.append(el)
    } else {
        element.prepend(el)
    }


    $("body").on("click", `.dropdown-${json["id"]}`, function (e) {
        e.stopPropagation();

        let dropdown = document.querySelector(`.dropdown-${json["id"]}`);
        dropdown.classList.toggle('is-active');
    });

}

/*

Ready

*/

$(document).ready(function () {
    var json = `{"offset": 0, "limit": 2}`
    $.ajax({
        url: "/api/post/posts",
        type: 'GET',
        dataType: 'json',
        data: json,
        contentType: 'application/json',
        processData: false,
        complete: function (xhr, text) {
            const code = xhr.status
            if (code == 500) {
                let parsed = JSON.parse(xhr.responseText)
                $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`)
                return
            }

            $(JSON.parse(xhr.responseText)).each(function (index, element) {
                add_post(element, true)
            });
        },
    });
});

/*

Submit events

*/

$("#post-form").submit(function (e) {
    e.preventDefault();

    const json = serialize_form(this);
    let j = JSON.parse(json);

    if (j["content"].length > 512 || j["content"].length == 0) {
        $("#post-errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
Content may not be more than 512 characters or empty.</div>
`)
        return
    }

    $.ajax({
        url: "/api/post/add",
        type: 'POST',
        dataType: 'json',
        contentType: 'application/json',
        processData: false,
        data: json,
        complete: function (xhr, text) {
            const code = xhr.status
            if (code == 400) {
                let parsed = JSON.parse(xhr.responseText)
                $("#post-errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`)
                return
            }
            if (code == 500) {
                let parsed = JSON.parse(xhr.responseText)
                $("#post-errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`)
                return
            }

            add_post(JSON.parse(xhr.responseText), false)
            $("#post-close").trigger("click");

        },
    });
});

$("#settings-form").submit(function (e) {
    e.preventDefault();
});

/*

Click event

*/

$("#post-button, #settings-button").on("click", function (e) {
    var val = $(this).attr("value");
    var element = $(`#${val}-modal`);
    element.addClass("flex");
    element.show();
});

$("#post-close, #settings-close").on("click", function (e) {
    e.preventDefault();

    var element = $(this).parent().parent().parent();
    element.removeClass("flex");
    element.hide();
});

$("body").on("click", "#error-close", function (e) {
    e.preventDefault();

    $(this).parent().remove();
});

$("body").on("click", "#dropdown-delete", function (e) {
    var value = $(this).attr('value');

    var json = `{"id": ${value}}`

    $.ajax({
        url: "/api/post/delete",
        type: 'DELETE',
        dataType: 'json',
        contentType: 'application/json',
        processData: false,
        data: json,
        complete: function (xhr, text) {
            const code = xhr.status
            if (code == 500) {
                let parsed = JSON.parse(xhr.responseText)
                $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`)
                return
            }

            $(`.post-${value}`).remove()

        },
    });
});