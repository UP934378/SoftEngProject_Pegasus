# Sprint 1 Plan

01/03 - 07/03

## Data Probe

Set up a simple data probe responds to network discovery requests and generates random data.

Key Points:
- Decide discovery protocol and identifier
- Decide transport protocol and data structure
- Don't worry about encryption yet
- See <https://github.com/ZacJW/GridTrace-DataProbe-ESP8266> for an example

##  Aggregator

Set up a simple aggregator that finds every data probe on the network and requests data from it at a fixed rate. It then parses the data and inserts it into the database.

Key Points:
- Use discovery protocol and identifier to find probes
- Write a parser to interpret the data that probes will send
- Use a worker pool (each worker is tasked with requesting data from a single probe)
- See <https://github.com/ZacJW/GridTrace/tree/master/gridtrace-data> for an example

## Database

Just handle the time-series data for now. This will mean becoming familiar with InFluxDB, both on the command line, and in application programming for it.

Key Points:
- Perform any required set up for InFluxDB to receive our data.
- Research how to use influx from Rust and write up a document explaining to whoever is
  doing the aggregator and the web app how to read/write data.

## Web App - Server side

Write a database access API to allow InFluxDB to be queried from HTTP. Also serve static files provided by whoever works on the client side.

Key Points:
- DB API should return JSON formatted data (work with client side on how best to structure it)
- DB API should have request options like time range or type
  (cell voltage, panel power, etc.) to filter output.
- Should serve static files that whoever does the client side will write.
- See <https://github.com/ZacJW/GridTrace/tree/master/gridtrace-web> for an example


## Web App - Client side

Write a web page that uses the database access API from the server side to pull data into a chart and keep it updated in real time.

Key Points:
- Choose a JS chart library to work with.
- Write some simple HTML and CSS for the page.
- Write JS to request from database API to populate chart.
- Any JS libraries should be downloaded so they can be served locally since the system
  shouldn't require internet to work.
- See <https://github.com/ZacJW/GridTrace/tree/master/usr/share/gridtrace/web> for an example
