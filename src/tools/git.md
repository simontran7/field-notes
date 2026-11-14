# Git

## Setting Up

- Initialize an existing directory as a Git repository

```console
$ git init
```

- Clone a specified remote repository

```syntax
git clone <remote repository url>
```

- Specifies the remote repository for your local repository.

```syntax
git remote add origin <remote repository url>
```

## Staging

- Show modified files in working directory, and files staged for your next commit

```console
$ git status
```

- Add a file as it looks now to your next commit

```syntax
git add <file 1> ... <file N>
```

- Unstage a file while retaining the changes in working directory

```syntax
git restore --staged <file 1> <...> <file N>
```

## Committing

- Commit the staged content as a new commit snapshot

```console
$ git commit 
```

> [!TIP]
> Scoped commit message style:
> ```syntax
> <scope>: <what changed>
> 
> <why the change was made or extra context>?
> 
> <trailers>?
> 
> <trailers>:
>     <key>: <value>
> ```

- Modify the previous commit's message

```syntax
git commit --amend -m "<new commit message>"
```

- Squashing commits

```syntax
git rebase -i <remote>/<branch>
```

Your text editor will pop up with a file looking as follows:

```
pick <hash> <message>
pick <hash> <message>
pick <hash> <message>
...
squash <hash> <message>
```

Change all the `pick` to squash except the first line as follows:

```
pick <hash> <message>
squash <hash> <message>
squash <hash> <message>
...
squash <hash> <message>
```

Save and close the file. 

Then, another text editor window will open which will let you combine the commit messages from all of the commits into a single commit message.

## Branching

- Display local branches

```console
$ git branch
```

- Display remote branches

```console
$ git branch -r 
```

- Create a branch 

```syntax
git branch <branch>
```

- Rename a branch

```syntax
git branch -m <branch to rename> <new branch name>
```

- Switch to a branch

```syntax
git switch <branch>
```

- Delete a branch

```syntax
git branch -d <branch>
```

- Delete a branch with unmerged changes

```syntax
git branch -D <branch>
```

- Delete a remote branch

```syntax
git push origin --delete <remote branch>
```

## Inspecting & Comparing

- Show the commit history for the current branch

```console
$ git log
```

- Show a condensed commit history as a graph, including all branches

```console
$ git log --oneline --graph --all
```

- Show unstaged changes in the working directory

```console
$ git diff
```

- Show staged changes (what will go into the next commit)

```console
$ git diff --staged
```

- Show the difference between two branches or commits

```syntax
git diff <branch or commit A> <branch or commit B>
```

- Show who last modified each line of a file, and in which commit

```syntax
git blame <file>
```

- Show the details and changes of a specific commit

```syntax
git show <commit>
```

- Move the `HEAD` pointer to a commit

```syntax
git switch --detach <commit> or HEAD<relative reference>
```

> [!WARNING]
> Checking out a commit causes the `HEAD` pointer to be in a detached head state, where any commits won't belong to any branches, and will be lost when you switch away (unless you run `git switch -c <new branch>` first to keep them).

## Merging & Rebasing

- Merge a branch into the current branch

```syntax
git merge <from branch>
```

- Replay the current branch's commits on top of a target branch

```syntax
git rebase <from branch>
```

> [!NOTE]
> Squashing is done inside an interactive rebase: mark commits with `squash` (or `s`) to combine them into the commit above.

- Abort a merge or rebase that has conflicts and return to the previous state

```console
$ git merge --abort
$ git rebase --abort
```

> [!TIP]
> 
> - Updating your private stuff from upstream: rebase
> - Landing your stuff into shared branches: merge

## Stashing 

- Save all staged and unstaged changes

```console
$ git stash
```

- List stack-order of stashed file changes

```console
$ git stash list
```

- Show a summary of the files changed in the most recent stash 

```syntax
git stash show stash@{<index>}
```

- Write the most recent stash back to working copy and removes it from the top of the stash stack

```console
$ git stash pop
```

- Write the most recent stash back to working copy and does _not_ remove it from the top of the stash stack

```console
$ git stash apply
```

- Discard the changes from top of stash stack

```console
$ git stash drop
```

## Undoing Things

- Move the current branch back to a commit, keeping changes staged (`--soft`), unstaged (`--mixed`, the default), or discarding them entirely (`--hard`)

```syntax
git reset [--soft | --mixed | --hard] <commit>
```

- Create a new commit that reverses the changes of a previous commit

```syntax
git revert <commit>
```

## Syncing

- Download commits and branches from the remote without changing your working directory

```syntax
git fetch <remote>
```

- Fetch from the remote and rebase the current branch's local commits on top of the remote branch, instead of merging

```console
$ git pull --rebase
```

## Pushing

- Push a branch to the remote 

```syntax
git push <remote> <branch>
```

- Push a branch that you've never pushed before to the remote

```syntax
git push -u origin <branch>
```