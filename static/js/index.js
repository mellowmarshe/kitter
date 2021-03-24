/*

Utils

*/

const serialize_form = form => JSON.stringify(
    Array.from(new FormData(form).entries())
    .reduce((m, [key, value]) => Object.assign(m, {
        [key]: value
    }), {})
);

/*

Submit events

*/

$("#post-form").on("submit", function (e) {
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

            window.location.href = `/`

        },
    });
});
$("#settings-form").on("submit", function (e) {
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

$("#post-close, #settings-close").on("click", function () {
    var element = $(this).parent().parent().parent();
    element.removeClass("flex");
    element.hide();
});

$("body").on("click", "#error-close", function () {
    $(this).parent().remove();
});

$("#dropdown-delete").on("click", function () {
    var value = $(this).attr('value');
    console.log(value);

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
            window.location.href = `/`

        },
    });
});