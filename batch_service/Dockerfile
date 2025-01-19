FROM clojure

WORKDIR /user/src/batch_service

COPY deps.edn .

RUN clojure -M:deps

COPY script.clj .

ENV PUSH_GATEWAY=http://localhost:9091

CMD ["clojure", "-M", "script.clj"]
