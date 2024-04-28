
##########################################################
# Dotenv
##########################################################

dotenv-pull: ## pull it down
	npx dotenv-vault@latest pull
	npx dotenv-vault@latest pull production
	npx dotenv-vault@latest pull example

dotenv-push: ## push yours up
	npx dotenv-vault@latest push
	npx dotenv-vault@latest push production

##########################################################
# Git
##########################################################

git-pull: ## git pull
	git pull origin

git-set-url: ## git remote set-url origin git@github.com:login/repo.git
	git remote set-url origin git@github.com:teratron/rustunumic.git

##########################################################
# Help
##########################################################

.PHONY: help
help:
	@awk '                                             \
		BEGIN {FS = ":.*?## "}                         \
		/^[a-zA-Z_-]+:.*?## /                          \
		{printf "\033[36m%-24s\033[0m %s\n", $$1, $$2} \
	'                                                  \
	$(MAKEFILE_LIST)

.DEFAULT_GOAL := help