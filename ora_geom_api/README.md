# Oracle Geometry Viewer API

## Run the project

Install dependencies with pip:

```bash
pip install -r requirements.txt
```

Create a `.env` file with the Oracle database information:

```bash
echo "DB_URL=localhost/BASE\nUSERNAME=sys\nPASSWORD=pass" >> .env
```

To run the project use `uvicorn`:

```bash
uvicorn main:app --reload
```

## Run with Docker

To run the project with Docker:

1. Create `.env.Dockerfile` where localhost is replaced with Docker's `host.docker.internal`:

    ```bash
    echo "DB_URL=host.docker.internal/BASE\nUSERNAME=sys\nPASSWORD=pass" >> .env.docker
    ```

2. Build the image:

    ```bash
    docker build -t geom_viewer_api .
    ```

3. Run the image:

    ```bash
    docker run --name geom_api --rm -p 8023:8023 --env-file .env.docker geom_viewer_api
    ```
