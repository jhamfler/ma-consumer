#!/bin/bash

echo started
echo brokername: $BROKERNAME
echo broker ip: $BROKERIP

if [ -z "$BROKERNAME" ]
then
	BROKERNAME=rabbitmq.default.svc.cluster.local
fi
echo brokername: "$BROKERNAME"

if [ -z "$BROKERIP" ]
then
	while IFS= read -r line
	do
		ips+=("$line")
	done < <(dig +short $BROKERNAME)

	ip=${ips[1]}
	echo found ip: ${ips[1]}
else
	ip=$BROKERIP
fi

echo starting consumer
consumer "$ip"":5672"
