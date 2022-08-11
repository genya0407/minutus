```shell-session
$ cargo ws version --force '*' --no-git-push patch
$ git push --tags
$ cargo ws publish --from-git
```
