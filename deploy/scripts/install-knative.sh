#!/bin/bash
# install knative via yaml operators.
# knative does not currently support helm installation.
cd "${0%/*}"

. ./common/check-env.sh
. ./common/error.sh
. ./common/print.sh

KNATIVE_VERSION=${KNATIVE_VERSION:=1.10.1}

print-header "installing knative version:$KNATIVE_VERSION"

check-env

kubectl apply -f "https://github.com/knative/serving/releases/download/knative-v${KNATIVE_VERSION}/serving-crds.yaml"
kubectl apply -f "https://github.com/knative/serving/releases/download/knative-v${KNATIVE_VERSION}/serving-core.yaml"
kubectl apply -f "https://github.com/knative/net-istio/releases/download/knative-v${KNATIVE_VERSION}/net-istio.yaml"

# TODO: Configuration of knative global feature maps to enable node selectors and security profiles etc.
