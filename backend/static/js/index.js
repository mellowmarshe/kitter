/*

Vars

*/

const posts_per = 25;
var current_offset = 0;
var post_count = 0;

// eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjb21wYW55IjoiS2l0dGVyIiwiZXhwIjoxNjE3NTgyMTM5LCJpc3MiOiI1IiwidXNlciI6eyJpZCI6NSwiZW1haWwiOiJkb21pbmljQGRvbW0ubWUiLCJ1c2VybmFtZSI6ImRvbXRlcmlvbiIsInBhc3N3b3JkIjoiZUkqeTBLOVVCSVVqaldoKmpAZUlHZ0hMIiwidGltZXN0YW1wIjoiMjAyMS0wMy0yOFQxOTozNTo1NC43ODE1NjBaIn19.O8wcYNaqF_FhbNAduh_yL-YL2G0A2IgyWMhbzvo2uQQ

var token = "";
/*

Utils

*/

const serialize_form = (form) =>
  JSON.stringify(
    Array.from(new FormData(form).entries()).reduce(
      (m, [key, value]) =>
        Object.assign(m, {
          [key]: value,
        }),
      {}
    )
  );

function isScrolledIntoView(elem) {
  var docViewTop = $(window).scrollTop();
  var docViewBottom = docViewTop + $(window).height();

  var elemTop = $(elem).offset().top;
  var elemBottom = elemTop + $(elem).height();

  return elemBottom <= docViewBottom && elemTop >= docViewTop;
}

function add_post(post, first) {
  var element = $("#posts");
  var json = post;
  var time = new Date(json["timestamp"]).toLocaleString();
  var hearts_html =
    json["author_id"] || 0 != 0
      ? `
<a class="level-item" id="heart-button" value="${json["id"]}" aria-label="heart">
    <span class="icon is-small">
        <i class="fa fa-heart is-black"></i>
    </span> <small class="p-1" id="hearts-count-${json["id"]}">${json["hearts"]}</small>
</a>
`
      : "";
  var del_html =
    json["author_id"] == parseInt(user)
      ? `
<div class="dropdown-item">
    <a href="#" class="dropdown-item" id="dropdown-delete"
        value="${json["id"]}">
        Delete
    </a>
</div>
`
      : "";

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
                                ${hearts_html}
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
`;

  if (first) {
    element.append(el);
  } else {
    element.prepend(el);
  }

  $("body").on("click", `.dropdown-${json["id"]}`, function (e) {
    e.stopPropagation();

    let dropdown = document.querySelector(`.dropdown-${json["id"]}`);
    dropdown.classList.toggle("is-active");
  });

  post_count += 1;
}

function get_pages(offset, limit) {
  var json = `{"offset": ${offset}, "limit": ${limit}}`;

  $.ajax({
    url: "/api/post/posts",
    type: "POST",
    dataType: "json",
    headers: {
      Authorization: `Bearer ${token}`,
    },
    contentType: "application/json",
    processData: false,
    data: json,
    complete: function (xhr, text) {
      const code = xhr.status;
      if (code == 500) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      if (code == 401) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      let json = JSON.parse(xhr.responseText);

      if (json.length == 0) {
        add_post(
          {
            id: 0,
            author: "SYSTEM",
            content: "No posts found....",
            hearts: 0,
            hearted_users: [],
            timestamp: Date.now(),
          },
          true
        );
        return;
      }

      current_offset += limit;

      $(json).each(function (index, element) {
        add_post(element, true);
      });
    },
  });
}

/*

Ready

*/

$(document).ready(function () {
  if (token) {
    get_pages(current_offset, posts_per);
  }
});

/*

Submit events

*/

$("#login-form").submit(function (e) {
  e.preventDefault();

  const json = serialize_form(this);
  let j = JSON.parse(json);

  $.ajax({
    url: "/api/user/login",
    type: "POST",
    dataType: "json",
    contentType: "application/json",
    processData: false,
    data: json,
    complete: function (xhr, text) {
      const code = xhr.status;
      if (code == 400) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      if (code == 401) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      if (code == 500) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      let parsed = JSON.parse(xhr.responseText);

      console.log(parsed["token"]);
    },
  });
});

$("#register-form").submit(function (e) {
  e.preventDefault();

  const json = serialize_form(this);
  let j = JSON.parse(json);

  $.ajax({
    url: "/api/user/register",
    type: "POST",
    dataType: "json",
    contentType: "application/json",
    processData: false,
    data: json,
    complete: function (xhr, text) {
      const code = xhr.status;
      if (code == 400) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      if (code == 401) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      if (code == 500) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      let parsed = JSON.parse(xhr.responseText);

      console.log(parsed);
    },
  });
});

$("#post-form").submit(function (e) {
  e.preventDefault();

  const json = serialize_form(this);
  let j = JSON.parse(json);

  if (j["content"].length > 512 || j["content"].length == 0) {
    $("#post-errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
Content may not be more than 512 characters or empty.</div>
`);
    return;
  }

  $.ajax({
    url: "/api/post/add",
    type: "POST",
    dataType: "json",
    contentType: "application/json",
    processData: false,
    data: json,
    headers: {
      Authorization: `Bearer ${token}`,
    },
    complete: function (xhr, text) {
      const code = xhr.status;
      if (code == 400) {
        let parsed = JSON.parse(xhr.responseText);
        $("#post-errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      if (code == 401) {
        let parsed = JSON.parse(xhr.responseText);
        $("#post-errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      if (code == 500) {
        let parsed = JSON.parse(xhr.responseText);
        $("#post-errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      add_post(JSON.parse(xhr.responseText), false);
      $("#post-content").val("");
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

$("#load-button").on("click", function (e) {
  e.preventDefault();
  get_pages(current_offset, posts_per);
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

$("body").on("click", "#heart-button", function (e) {
  e.preventDefault();

  var value = $(this).attr("value");

  var json = `{"id": ${value}}`;

  $.ajax({
    url: "/api/post/heart",
    type: "POST",
    dataType: "json",
    contentType: "application/json",
    processData: false,
    data: json,
    headers: {
      Authorization: `Bearer ${token}`,
    },
    complete: function (xhr, text) {
      const code = xhr.status;
      if (code == 500) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}.. that post may have been deleted.</div>
`);
        return;
      }

      if (code == 401) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      let json = JSON.parse(xhr.responseText);

      $(`#hearts-count-${value}`).text(json["hearts"]);
    },
  });
});

$("body").on("click", "#dropdown-delete", function (e) {
  var value = $(this).attr("value");

  var json = `{"id": ${value}}`;

  $.ajax({
    url: "/api/post/delete",
    type: "DELETE",
    dataType: "json",
    contentType: "application/json",
    processData: false,
    data: json,
    headers: {
      Authorization: `Bearer ${token}`,
    },
    complete: function (xhr, text) {
      const code = xhr.status;
      if (code == 500) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      if (code == 401) {
        let parsed = JSON.parse(xhr.responseText);
        $("#errors").append(`<div class="notification error">
<button class="delete" id="error-close"></button>
${parsed["error"]}</div>
`);
        return;
      }

      $(`.post-${value}`).remove();
    },
  });
});

/*

Others

*/

$(window).scroll(function () {
  if (isScrolledIntoView($("#load-button"), false)) {
    get_pages(current_offset, posts_per);
  }
});