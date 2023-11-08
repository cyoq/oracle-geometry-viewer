FROM python:3.11

WORKDIR /app

COPY requirements.txt /tmp/requirements.txt

COPY main.py main.py

RUN pip install -r /tmp/requirements.txt && rm -rf /tmp

CMD uvicorn main:app --host 0.0.0.0 --port 8023