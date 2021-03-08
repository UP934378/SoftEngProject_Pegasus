# Sprint 2 Plan

08/03 - 14/03

This week is mainly about introducing login and configuration to the project,
but also about introducing encryption to the aggregation layer.

## Data Probe

- Refactor to use HTTPS webserver
- Refactor WiFi creds to be stored in SPIFFS

##  Aggregator

- Use the probe config table to configure workers
- Key exchange mechanism and certificate pinning
- Formalise error logging and communicate with maintenance system

## Database

- Add probe config table to DB
- Add user table to DB
- Add notification rules table to DB

## Maintenance System

- Create workers that communicate with each connected sub-system
- Research email and push notifications and build a simple demo

## Web App - Server side

- Add user authentication
- Formalise error logging and communicate with maintenance system
- Add probe config API that checks that the user is logged in as admin and allows config get/set

## Web App - Client side

- Write login page
- Refactor visualisation code to be more flexible (object orientated)
- Write probe management page that uses the probe config API

## Everything

- Look at how the code can be organised to make it more testable
- Write tests (look at using [proptest](https://docs.rs/proptest/0.9.4/proptest/))
- Once I've finished adding code coverage reports, try to maximise test coverage.
