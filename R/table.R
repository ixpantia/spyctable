#' @export
renderSpyCTable <- function(expr, env = parent.frame(), quoted = FALSE) {
  func <- shiny::exprToFunction(expr, env, quoted)
  shiny::reactive({
    to_render <- func()
    list(
      data = to_render,
      thead = jsonlite::unbox(spyc_header_create(colnames(to_render)))
    )
  })
}

#' @export
spyCTableOutput <- function(id, scroll_y = "50vh") {
  shiny::tagList(
    htmltools::htmlDependency(
      name = "spyctable",
      version = "0.1.0",
      package = "spyctable",
      src = "table",
      script = "index.js"
    ),
    htmltools::htmlDependency(
      name = "datatables",
      version = "2.0.8",
      package = "spyctable",
      src = "datatables",
      script = "datatables.min.js",
      stylesheet = "datatables.min.css"
    ),
    shiny::div(
      class = "spyctable",
      id = id,
      `scroll-y` = scroll_y
    )
  )
}

#' @export
get_spyc_table_selection <- function(input, dataset) {
  filter_from_values_vec(as.integer(input), dataset)
}
