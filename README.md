# Oracle SDO Geometry viewer

A university project to create a viewer of SDO geometry objects of the Oracle database. For this project, I have chosen to use Rust and [egui](https://github.com/emilk/egui/tree/master) library for GUI development.

Unfortunately, [rust-oracle](https://github.com/kubo/rust-oracle) uses a thick ODPI-C client that is not available on Mac [ARM computers](https://stackoverflow.com/q/74225139). It is also quite troublesome to install the ODPI-C client via [Rosetta](https://developers.ascendcorp.com/how-to-install-oracle-instant-client-on-apple-silicon-m1-24b67f2dc743) or Docker.

Therefore, I decided to separate the GUI from database queries and instead make requests to the backend that uses a thin database client. I found two libraries that allow thin clients [node-oracledb](https://github.com/oracle/node-oracledb) and [python-oracledb](https://github.com/oracle/python-oracledb). Thin clients directly connect to the database without the [help of the driver](https://medium.com/oracledevs/usher-in-a-new-era-with-the-node-oracledb-6-0-pure-javascript-thin-driver-e10e2af693b2). One more option was to use Java and an Oracle official JAR file, but I just did not want to work with Java :smile:.

I chose to use `python-oracledb` because I am familiar with it and I wanted to try [FastAPI](https://fastapi.tiangolo.com/).

## Inspirations

The project shares a good part of code from [creativecode/headlines](https://github.com/creativcoder/headlines/tree/ep7b) and from egui official [demo](https://www.egui.rs/#Demo) project.

## Running the project

To run each part, please, refer to the corresponding README files in the folders: [ora_geom_gui](./ora_geom_gui/README.md) and [ora_geom_api](./ora_geom_api/README.md)

## Process description

Communication between services is going to be pretty straightforward. GUI connects to the backend and the backend connects to the database. GUI does not do any actions with the database and it just uses a middle-man to get results to draw. **Be aware** that it is not going to be a production-ready product, but it is more of a learning project where security is in the last place, so no security checks will be done on the backend side. In real environments when we get raw SQL queries, security checks must be done.

The communication diagram is displayed below. As was mentioned, the Backend stays as a middle-man between database and GUI. GUI's goal is to display results, Backend's goal is to get those results and transform them into an understandable format and the database just stores results.

```mermaid
stateDiagram-v2
    direction LR
    GUI --> Backend : Sends a query request
    Backend --> Database : Sends a query request
    Database --> Backend : Sends a result set or an error
    Backend --> Backend : Transforms a result set
    Backend --> GUI : Sends a transformed result set or an error
```

### Communication scenarios

In the following diagram, a successful scenario is displayed:

```mermaid
sequenceDiagram
    autonumber
    GUI->>+API: Send a query request
    API->>+Database: Send a query
    Database -->>- API : Send a result set
    API ->> API : Transform a result set to JSON
    API-->>-GUI: Send a result set
    GUI ->> GUI : Display a geometry
```

In the next diagram, a user sends an incorrect SQL query that does not query any geometry data:

```mermaid
sequenceDiagram
    GUI->>+API: Send a query request
    API->>+Database: Send a query
    Database -->>- API : Send a result set
    API ->> API : Transform a result set to JSON
    alt a result set does not contain any geometry data
        API-->>GUI: Send an error message that there is no geometry data
        GUI ->> GUI : Display an error message to a user
    else otherwise
        API-->>-GUI: Send a result set
    end
    GUI ->> GUI : Display a geometry
```

In the last scenario diagram, a user sends a SQL query that contains any syntax error:

```mermaid
sequenceDiagram
    GUI->>+API: Send a query request
    API->>+Database: Send a query
    alt a query contains a syntax error
        Database-->>API: Send an error message that there is a syntax error
        API ->> GUI : Send an error message that there is a syntax error
        GUI ->> GUI : Display an error to a user
    else otherwise
        Database -->>- API : Send a result set
    end
    API ->> API : Transform a result set to JSON
    API-->>-GUI: Send a result set
    GUI ->> GUI : Display a geometry
```

## An example of geometry data in Oracle DB

Here is an example of an Oracle geometry data:

```sql
SELECT
    GEOMETRY
FROM 
    BUILDINGS;
```

| GEOMETRY                                                                                                                   |
| -------------------------------------------------------------------------------------------------------------------------- |
| {"SDO_GTYPE":2003,"SDO_SRID":null,"SDO_POINT":{},"SDO_ELEM_INFO":[1,3,1],"SDO_ORDINATES":[40,23,48,23,48,29,40,29,40 ,23]} |
| {"SDO_GTYPE":2003,"SDO_SRID":null,"SDO_POINT":{},"SDO_ELEM_INFO":[1,3,1],"SDO_ORDINATES":[27,0,34,0,34,6,27,6,27,0]}       |
| {"SDO_GTYPE":2003,"SDO_SRID":null,"SDO_POINT":{},"SDO_ELEM_INFO":[1,3,1],"SDO_ORDINATES":[34,0,48,0,48,6,34,6,34,0]}       |

*Note* that geometry viewer can only display 3 types of geometries: lines, polygons and circles.

## Demo

In the video below you can see how the program works in action:

![demo video](./misc/demo.mov)
