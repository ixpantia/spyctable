library(spyctable)
library(shiny)
library(tidyselect)
library(dplyr)

ui <- fluidPage(
  theme = bslib::bs_theme(version = 5),
  actionButton("rerender", "Rerender"),
  spyCTableOutput("tabla")
)

server <- function(input, output, session) {

  output$tabla <- renderSpyCTable({
    iris |>
      dplyr::mutate(Species = as.character(Species))
  }) |>
    bindEvent(input$rerender)

  observe({
    print(
      get_spyc_table_selection(input$tabla_cells_selected, iris)
    )
  })
}

shinyApp(ui, server)
