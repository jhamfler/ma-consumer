FROM ubuntu

COPY target/debug/consumer /usr/local/bin/
#COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/consumer
#RUN chmod +x /usr/local/bin/docker-entrypoint.sh
ENTRYPOINT ["consumer"]
CMD []
#EXPOSE 21
