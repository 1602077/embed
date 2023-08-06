#!/bin/bash

function check-env() {
	K8S_CONTEXT=$(kubectl config current-context)

	read -p "pushing to context '$K8S_CONTEXT'. Are you sure? y/n " yn
	case $yn in
	[Yy]*) ;;
	*) exit 1 ;;
	esac
}
