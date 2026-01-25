FROM python:3.11-alpine

RUN pip install supervisor
COPY supervisord.conf /etc/supervisord.conf
COPY entrypoint.sh /entrypoint.sh
COPY programs /app
RUN chmod +x entrypoint.sh /app/*
RUN mkdir /etc/supervisor.d

ENTRYPOINT /entrypoint.sh
CMD tail -f /dev/null
