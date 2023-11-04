from fastapi import FastAPI

app = FastAPI(docs_url="/swagger")


@app.get("/healthcheck", description="Check if API is working")
async def healthcheck():
    return "OK"
