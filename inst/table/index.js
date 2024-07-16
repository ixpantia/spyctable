var spyCTableBinding = new Shiny.OutputBinding();

spyCTableBinding.find = function(scope) {
  return $(scope).find(".spyctable");
}

var globalSpyCTableIndex = new Map();
var spyCTableSelectionBuffer = new Array();

// Its its true then it's dragging
var is_dragging = false;

function disable_dragging() {
  is_dragging = false;
}

function enable_dragging() {
  is_dragging = true;
}

function selected_deselected(el) {
  // This is a pointer to the selection array 
  const selection = globalSpyCTableIndex.get(el.tableId);
  let is_selected = el.classList.contains('selected');
  if (is_selected) {
    selection.delete(el);
    el.classList.remove("bg-primary");
    el.classList.remove("selected");
  } else {
    selection.add(el);
    el.classList.add("bg-primary");
    el.classList.add("selected");
  }
  spyCTableSelectionBuffer.length = 0;
  for (const element of selection) {
    spyCTableSelectionBuffer.push(element.coords);
  }
  Shiny.setInputValue(el.inputId, spyCTableSelectionBuffer);
}

function mouse_over_event() {
  if (is_dragging) {
    selected_deselected(this)
  }
}

function mouse_down_event() {
  selected_deselected(this)
}

// This function is to deselect everything in the table
function spyctable_deselect_all(tableId) {
  const selection = globalSpyCTableIndex.get(tableId);
  if (selection !== undefined) {
    for (const element of selection) {
      element.classList.remove("bg-primary");
      element.classList.remove("selected");
    }
    selection.clear();
    spyCTableSelectionBuffer.length = 0;
    Shiny.setInputValue(el.inputId, spyCTableSelectionBuffer);
  }
}

// If anywhere on the page the mouseup event is found
// then we disable dragging
addEventListener("mouseup", (_event) => {
  disable_dragging();
});

function build_tbody(tableId, inputId, len_x, len_y, data, keys) {
  var tbody = document.createElement("tbody");

  // If the user clicks then we enable dragging
  tbody.onmousedown = enable_dragging;

  // If the user's mouse leaves the table we disable dragging
  tbody.onmouseleave = disable_dragging;

  // Just in case, if the mouse just entered the table we
  // disable dragging aswell
  tbody.onmouseenter = disable_dragging;

  const fragment = document.createDocumentFragment();

  for (var i = 0; i < len_y; i++) {
    var current_row = document.createElement("tr");
    for (var c = 0; c < len_x; c++) {
      var current_cel = document.createElement("td");
      current_cel.coords = [c, i];
      current_cel.innerText = data[keys[c]][i];
      current_cel.classList.add("user-select-none");
      //We passed the pointer to every single cell 
      current_cel.tableId = tableId;
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

  var selection = globalSpyCTableIndex.get(id);
  if (selection === undefined) {
    selection = new Set();
    globalSpyCTableIndex.set(id, selection);
  }
  let data = msg.data;
  let thead_content = msg.thead;
  let keys = Object.keys(data);
  let len_x = keys.length;
  let len_y = data[keys[0]].length;
  var table = document.createElement("table");
  table.classList.add("table");
  table.id = id + '_inner_table';
  table.appendChild(fromHTML(thead_content));
  table.appendChild(build_tbody(id, inputId, len_x, len_y, data, keys));
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
