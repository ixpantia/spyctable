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
  const selection = globalSpyCTableIndex.get(el.dataset.table_id);
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
    spyCTableSelectionBuffer.push(element.dataset.coords);
  }
  Shiny.setInputValue(el.dataset.table_id + '_cells_selected', spyCTableSelectionBuffer);
}

function mouse_over_event(el) {
  if (is_dragging) {
    selected_deselected(el)
  }
}

function mouse_down_event(el) {
  selected_deselected(el)
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
    Shiny.setInputValue(tableId + '_cells_selected', spyCTableSelectionBuffer);
  }
}

// If anywhere on the page the mouseup event is found
// then we disable dragging
addEventListener("mouseup", (_event) => {
  disable_dragging();
});

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

  var table = fromHTML(msg.html);
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
