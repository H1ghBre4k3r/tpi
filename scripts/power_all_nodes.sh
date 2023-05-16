#!/bin/sh

curl --location --request POST 'http://127.0.0.1/api/bmc?opt=set&type=power&node1=1&node2=1&node3=1&node4=1'
status=$?

while [ "$status" -ne 0 ]; do 
    sleep 10
    curl --location --request POST 'http://127.0.0.1/api/bmc?opt=set&type=power&node1=1&node2=1&node3=1&node4=1'
    status=$?
done
