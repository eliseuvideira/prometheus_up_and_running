(ns batch-service.script
  (:require
   [iapetos.core :as prometheus]
   [iapetos.export :as prometheus-export]))

(def push-gateway-url
  (clojure.string/replace
   (or (System/getenv "PUSH_GATEWAY_URL") "http://localhost:9091")
   #"^https?://" ""))

(def instance-name
  (or (System/getenv "INSTANCE_NAME") "batch-service"))

(defonce registry
  (-> (prometheus-export/pushable-collector-registry
       {:push-gateway push-gateway-url
        :job "batch-service"
        :grouping-key {"instance" instance-name}})
      (prometheus/register
       (prometheus/histogram :app/duration-seconds)
       (prometheus/counter   :app/runs-total)
       (prometheus/gauge :app/memory-usage)
       (prometheus/gauge :app/cpu-usage))))

(defn batch-service []
  (let [start-time (System/currentTimeMillis)
        _         (Thread/sleep 1000)
        duration  (/ (- (System/currentTimeMillis) start-time) 1000.0)]

    (-> registry
        (prometheus/observe :app/duration-seconds duration)
        (prometheus/inc :app/runs-total)
        (prometheus/set :app/memory-usage (rand 100))
        (prometheus/set :app/cpu-usage (rand 100))
        (prometheus-export/push!)))

  (println "Batch service finished"))

(defn -main []
  (batch-service))

(-main)
