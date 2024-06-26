library(spyctable)
library(shiny)
library(tidyselect)
library(dplyr)

ui <- fluidPage(
  theme = bslib::bs_theme(version = 5),
  actionButton("rerender", "Rerender"),
  spyCTableOutput("tabla")
)

char_iris <- 1:100 |>
  purrr::map_df(~ iris) |>
  dplyr::mutate(dplyr::across(tidyselect::everything(), as.character))

server <- function(input, output, session) {

  output$tabla <- renderSpyCTable({
    char_iris
  }) |>
    bindEvent(input$rerender)

  observe({
    print(
      get_spyc_table_selection(input$tabla_cells_selected, char_iris)
    )
  })
}

shinyApp(ui, server)
