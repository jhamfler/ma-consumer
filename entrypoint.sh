#!/bin/bash

echo started
echo brokername: $BROKERNAME
echo broker ip: $BROKERIP

if [ -z "$BROKERNAME" ]
then
	BROKERNAME=rabbitmq.default
fi
echo brokername: "$BROKERNAME"

if [ -z "$BROKERIP" ]
then
	ip=$BROKERIP
else
	while IFS= read -r line
	do
		ips+=("$line")
	done < <(dig +short $BROKERNAME)

	ip=${ips[1]}
	echo found ip: ${ips[1]}
fi

echo starting consumer
consumer "$ip"":5672"
