#!/bin/bash
ips=()
ip=

echo started
echo brokername: $BROKERNAME
echo broker ip: $BROKERIP

if [ -z "$BROKERNAME" ]
then
	BROKERNAME=rabbitmq.default.svc.cluster.local
	#BROKERNAME=web.de
fi
echo brokername: "$BROKERNAME"

if [ -z "$BROKERIP" ]
then
	ips=($(dig +short $BROKERNAME))
	ips=("${ips[@]%%:*}")

	ip=$ips
	echo found ip: $ips
else
	ip=$BROKERIP
fi

echo starting consumer
consumer "$ip"":5672"
