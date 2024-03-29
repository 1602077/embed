#!/bin/bash
# Creates workload identity pool for GitHub actions connection to artefact
# registry & GKE. Eventually this would be set-up by terraform.
cd "${0%/*}"

. ./common/check-env.sh
. ./common/error.sh
. ./common/print.sh

PROJECT_ID=${PROJECT_ID}
GITHUB_REPO=${GITHUB_REPO} # username/repo
SERVICE_ACCOUNT=${SERVICE_ACCOUNT:="github-ci"}
WORKLOAD_IDENTITY_POOL=${WORKLOAD_IDENTITY_POOL:="github-ci"}
WORKLOAD_PROVIDER=${WORKLOAD_PROVIDER:="github-ci-provider"}

# create service account and assign artifact registry writer permissions.
gcloud iam service-accounts create "${SERVICE_ACCOUNT}" \
	--project "${PROJECT_ID}"

gcloud projects add-iam-policy-binding $PROJECT_ID \
	--member="serviceAccount:${SERVICE_ACCOUNT}@${PROJECT_ID}.iam.gserviceaccount.com" \
	--role="roles/artifactregistry.writer"

gcloud services enable iamcredentials.googleapis.com --project "${PROJECT_ID}"

gcloud iam workload-identity-pools create "${WORKLOAD_IDENTITY_POOL}" \
	--project="${PROJECT_ID}" \
	--location="global" \
	--display-name="${WORKLOAD_IDENTITY_POOL}"

WORKLOAD_IDENTITY_POOL_ID=$(
	gcloud iam workload-identity-pools describe "${WORKLOAD_IDENTITY_POOL}" \
		--project="${PROJECT_ID}" \
		--location="global" \
		--format="value(name)"
)

gcloud iam workload-identity-pools providers create-oidc "${WORKLOAD_PROVIDER}" \
	--project="${PROJECT_ID}" \
	--location="global" \
	--workload-identity-pool="${WORKLOAD_IDENTITY_POOL}" \
	--display-name="${WORKLOAD_PROVIDER}" \
	--attribute-mapping="google.subject=assertion.sub,attribute.actor=assertion.actor,attribute.repository=assertion.repository" \
	--issuer-uri="https://token.actions.githubusercontent.com"

gcloud iam service-accounts add-iam-policy-binding "${SERVICE_ACCOUNT}@${PROJECT_ID}.iam.gserviceaccount.com" \
	--project="${PROJECT_ID}" \
	--role="roles/iam.workloadIdentityUser" \
	--member="principalSet://iam.googleapis.com/${WORKLOAD_IDENTITY_POOL_ID}/attribute.repository/${GITHUB_REPO}"

WORKLOAD_IDENITY_PROVIDER_ID=$(
	gcloud iam workload-identity-pools providers describe "${WORKLOAD_PROVIDER}" \
		--project="${PROJECT_ID}" \
		--location="global" \
		--workload-identity-pool="${WORKLOAD_IDENTITY_POOL}" \
		--format="value(name)"
)

print-header "identity pool created with provider '${WORKLOAD_IDENITY_PROVIDER_ID}'"
