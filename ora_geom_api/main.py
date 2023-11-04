import enum
from typing import Any
from fastapi import FastAPI, HTTPException, status
import oracledb
from pydantic import BaseModel
from pydantic_settings import BaseSettings


app = FastAPI(docs_url="/swagger")


class Settings(BaseSettings):
    DB_URL: str
    USERNAME: str
    PASSWORD: str

    class Config:
        env_file = ".env"


settings = Settings()


@app.get("/healthcheck", description="Check if API is working")
async def healthcheck():
    return "OK"


def execute_sql(sql: str) -> list[Any]:
    with oracledb.connect(
        user=settings.USERNAME,
        password=settings.PASSWORD,
        dsn=settings.DB_URL,
        mode=oracledb.SYSDBA,
    ) as conn:
        with conn.cursor() as cursor:
            cursor.execute(sql)
            return cursor.fetchmany()


class Validation(enum.Enum):
    Ok = 0
    EmptySet = 1
    TooManyElements = 2
    NotGeometryData = 3


def validate_data(data: list[Any]) -> Validation:
    if len(data) == 0:
        return Validation.EmptySet

    # Check column amount
    if len(data[0]) > 1:
        return Validation.TooManyElements

    # Check if it is a geometry object
    element = data[0][0]
    if type(element) != oracledb.DbObject or element.type.name != "SDO_GEOMETRY":
        return Validation.NotGeometryData

    return Validation.Ok


class SdoGeometry(BaseModel):
    sdo_gtype: float
    sdo_srid: float | None
    sdo_point: dict | None
    sdo_elem_info: list[float]
    sdo_ordinates: list[float]


def convert_data(data: list[oracledb.DbObject]) -> list[SdoGeometry]:
    # Get one sdo object from the tuple
    sdo_objects = [row[0] for row in data]
    return [
        SdoGeometry(
            sdo_gtype=sdo_object.SDO_GTYPE,
            sdo_srid=sdo_object.SDO_SRID,
            sdo_point=sdo_object.SDO_POINT,
            sdo_elem_info=sdo_object.SDO_ELEM_INFO.aslist(),
            sdo_ordinates=sdo_object.SDO_ORDINATES.aslist(),
        )
        for sdo_object in sdo_objects
    ]


class GeometryRequest(BaseModel):
    sql: str


class Model400(BaseModel):
    detail: str = "Error message"


@app.post(
    "/geometry",
    description="Send a geometry query to the Oracle database",
    responses={400: {"model": Model400}},
    response_model=list[SdoGeometry],
)
async def geometry(request: GeometryRequest) -> list[SdoGeometry]:
    if "select" not in request.sql.lower():
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Only SELECT statements can be used in a query",
        )

    data: list = execute_sql(request.sql)

    match validate_data(data):
        case Validation.EmptySet:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="No data provided from the query",
            )
        case Validation.TooManyElements:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="Please, query only one geometry column",
            )
        case Validation.NotGeometryData:
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="Please, query ony column with SDO_GEOMETRY object",
            )

    return convert_data(data)
