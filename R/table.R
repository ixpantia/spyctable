#' @export
renderIxTable <- function(expr, env = parent.frame(), quoted = FALSE) {
  func <- shiny::exprToFunction(expr, env, quoted)
  shiny::reactive({
    to_render <- func()
    list(
      data = to_render,
      thead = jsonlite::unbox(ixt_header_create(colnames(to_render)))
    )
  })
}

#' @export
ixTableOutput <- function(id, scroll_y = "50vh") {
  shiny::tagList(
    htmltools::htmlDependency(
      name = "ixTable",
      version = "0.1.0",
      package = "ixtable",
      src = "table",
      script = "index.js"
    ),
    htmltools::htmlDependency(
      name = "datatables",
      version = "2.0.8",
      package = "ixtable",
      src = "datatables",
      script = "datatables.min.js",
      stylesheet = "datatables.min.css"
    ),
    shiny::div(
      class = "ix-table",
      id = id,
      `scroll-y` = scroll_y
    )
  )
}

#' @export
get_ixtable_selection <- function(input, dataset) {
  filter_from_values_vec(as.integer(input), dataset)
}
