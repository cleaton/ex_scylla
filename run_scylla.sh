#!/bin/bash
docker run --net=host --name exscylla scylladb/scylla:6.2 --developer-mode 1 --listen-address 127.0.0.1