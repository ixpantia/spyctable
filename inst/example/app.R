library(spyctable)
library(shiny)
library(tidyselect)
library(dplyr)

iris_char <- iris |>
  dplyr::mutate(Species = as.character(Species))

table_module_ui <- function(id) {
  ns <- shiny::NS(id)
  spyCTableOutput(ns("tabla"))
}

ui <- fluidPage(
  theme = bslib::bs_theme(version = 5),
  table_module_ui("my_module")
)

table_module_server <- function(id) {
  shiny::moduleServer(id, function(input, output, session) {
    output$tabla <- renderSpyCTable({
      spyctable(
        iris_char,
        format = "default",
        na = "dash"
      )
    })

    observe({
      print(
        get_spyc_table_selection(input$tabla_cells_selected, iris_char)
      )
    })

  })
}


server <- function(input, output, session) {

  table_module_server("my_module")

}

shinyApp(ui, server)
