#!/bin/bash
docker run --net=host --name exscylla scylladb/scylla:5.1.0-rc4  --developer-mode 1 --listen-address 127.0.0.1