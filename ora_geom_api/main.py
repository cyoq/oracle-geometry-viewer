from fastapi import FastAPI
from pydantic import BaseModel


app = FastAPI(docs_url="/swagger")


@app.get("/healthcheck", description="Check if API is working")
async def healthcheck():
    return "OK"


class GeometryRequest(BaseModel):
    sql: str


@app.post("/geometry", description="Send a geometry query to the Oracle database")
async def geometry(query: GeometryRequest):
    return "OK"
