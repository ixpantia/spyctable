#' @export
spyctable <- function(data, format = "default", na = "zero") {
  list(
    data = data,
    format = format,
    na = na
  )
}

#' @export
renderSpyCTable <- function(expr, env = parent.frame(), quoted = FALSE, id) {
  func <- shiny::exprToFunction(expr, env, quoted)
  session <- shiny::getDefaultReactiveDomain()
  shiny::reactive({
    to_render <- func()
    list(
      html = jsonlite::unbox(
        build_spyctable_html(
          to_render$data,
          colnames(to_render$data),
          nrow(to_render$data),
          to_render$format,
          to_render$na,
          id = shiny::getCurrentOutputInfo()$name
        )
      )
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
  filter_from_values_vec(as.character(input), dataset)
}
