# Git Hooks

Reassigning the location of hooks:

```shell
git config --global core.hooksPath .githooks
```

Prints all git global settings to the console,
after changing the location of git hooks,
in the settings `core.hookspath=.githooks` appears:

```shell
git config --global --list
```

Returns the default value of the `core.hookspath` config:

```shell
git config --global --unset core.hooksPath
```

## Commit

1. pre-commit
2. prepare-commit-msg
3. commit-msg

## Push

1. pre-push
