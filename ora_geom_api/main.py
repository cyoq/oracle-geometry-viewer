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


class GeometryRequest(BaseModel):
    sql: str


class Model400(BaseModel):
    detail: str = "Only SELECT statements can be used in a query"


@app.post(
    "/geometry",
    description="Send a geometry query to the Oracle database",
    responses={400: {"model": Model400}},
)
async def geometry(request: GeometryRequest):
    if "select" not in request.sql.lower():
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Only SELECT statements can be used in a query",
        )

    with oracledb.connect(
        user=settings.USERNAME,
        password=settings.PASSWORD,
        dsn=settings.DB_URL,
        mode=oracledb.SYSDBA,
    ) as conn:
        with conn.cursor() as cursor:
            cursor.execute(request.sql)
            rows = cursor.fetchmany()
            print("%s", rows)
    return "OK"
