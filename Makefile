.PHONY: document install example

document: 
	Rscript -e "rextendr::document()"

install:
	Rscript -e "devtools::install()"

example:
	cd inst/example && Rscript -e "shiny::runApp(port = 3838)"
