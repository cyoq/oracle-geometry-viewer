# Oracle Geometry Viewer API

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
