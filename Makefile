redis-server:
	redis-server --loglevel warning --protected-mode no --loadmodule /opt/redis-stack/lib/rejson.so --loadmodule /opt/redis-stack/lib/redisearch.so