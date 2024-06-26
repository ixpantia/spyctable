var spyCTableBinding = new Shiny.OutputBinding();

spyCTableBinding.find = function(scope) {
  return $(scope).find(".spyctable");
}

// Its its true then it's dragging
var is_dragging = false;

addEventListener("mousedown", (_event) => {
  is_dragging = true;
});

addEventListener("mouseup", (_event) => {
  is_dragging = false;
});

function selected_deselected(el) {
  const selection = el.selection;
  let is_selected = el.classList.contains('selected');
  if (is_selected) {
    selection.delete(el.coords);
    el.classList.remove("bg-primary");
    el.classList.remove("selected");
  } else {
    selection.add(el.coords);
    el.classList.add("bg-primary");
    el.classList.add("selected");
  }
  Shiny.setInputValue(el.inputId, Array.from(selection))
}

function mouse_over_event() {
  if (is_dragging) {
    selected_deselected(this)
  }
}

function mouse_down_event() {
  selected_deselected(this)
}

function build_tbody(selection, inputId, len_x, len_y, data, keys) {
  var tbody = document.createElement("tbody");

  const fragment = document.createDocumentFragment();

  for (var i = 0; i < len_y; i++) {
    var current_row = document.createElement("tr");
    for (var c = 0; c < len_x; c++) {
      var current_cel = document.createElement("td");
      current_cel.coords = [c, i];
      current_cel.innerText = data[keys[c]][i];
      current_cel.classList.add("user-select-none");
      current_cel.selection = selection;
      current_cel.onmouseover = mouse_over_event;
      current_cel.onmousedown = mouse_down_event;
      current_cel.inputId = inputId;
      current_row.appendChild(current_cel);
    }
    fragment.appendChild(current_row);
  }

  tbody.appendChild(fragment);

  return tbody;
}

function fromHTML(html, trim = true) {
  // Process the HTML string.
  html = trim ? html.trim() : html;
  if (!html) return null;

  // Then set up a new template element.
  const template = document.createElement('template');
  template.innerHTML = html;
  const result = template.content.children;

  // Then return either an HTMLElement or HTMLCollection,
  // based on whether the input HTML had one or more roots.
  if (result.length === 1) return result[0];
  return result;
}

spyCTableBinding.renderValue = function(el, msg) {

  let id = el.id;
  let inputId = id + '_cells_selected';

  if (el.datatable) {
    el.datatable.destroy();
    el.innerHTML = '';
    Shiny.setInputValue(inputId, new Array())
  }

  let data = msg.data;
  let thead_content = msg.thead;
  el.selection = new Set();
  let keys = Object.keys(data);
  let len_x = keys.length;
  let len_y = data[keys[0]].length;
  var table = document.createElement("table");
  table.classList.add("table");
  table.id = id + '_inner_table';
  table.appendChild(fromHTML(thead_content));
  table.appendChild(build_tbody(el.selection, inputId, len_x, len_y, data, keys));
  el.appendChild(table);

  let scroll_y = el.getAttribute("scroll-y");

  let datatable = new DataTable('#' + table.id, {
    scrollY:        scroll_y,
    deferRender:    true,
    scroller:       true
  });

  el.datatable = datatable;

}

Shiny.outputBindings.register(spyCTableBinding, "spyCTableBinding");
