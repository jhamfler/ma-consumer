#!/bin/bash

echo started
echo brokername: $BROKERNAME

if [ -z "$BROKERNAME" ]
then
	$BROKERNAME=rabbitmq.default
fi

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

consumer "$ip"":5672"
