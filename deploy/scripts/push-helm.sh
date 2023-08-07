#!/bin/bash
# Deploys helm charts via helmfile and installs knative via yaml operators.
# Used to set-up a fresh cluster ready for application deployments.
# usage: HELMFILE_ENV={{ HELMFILE_ENV }} ./deploy/scripts/push-helm.sh
cd "${0%/*}"

. ./common/check-env.sh
. ./common/error.sh
. ./common/print.sh

# Override helm environment as necessary with environment variable.
HELMFILE_ENV=${HELMFILE_ENV:=dev}

print-header "deploying helm charts with $HELMFILE_ENV environment configuration, tier: architecture"

check-env

(
	cd .. # i.e. root of './deploy'.
	helmfile -e "$HELMFILE_ENV" deps
	helmfile -e $HELMFILE_ENV sync --selector tier=architecture
)

./install-knative.sh

# TODO: waiting for knative to be healthy

print-header "deploying helm charts with $HELMFILE_ENV environment configuration, tier: application"

check-env

(
	cd .. # i.e. root of './deploy'.
	helmfile -e $HELMFILE_ENV sync --selector tier=application
)

print-header "base cluster environment configured"
