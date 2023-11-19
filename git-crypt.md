# Background
The issue to [not publish your input files](https://www.reddit.com/r/adventofcode/comments/zh2hk0/2022friendly_reminder_dont_commit_your_input/) came up on Reddit.

There are multiple ways to honor this request, such as:
- Using a submodule for the `input` directory, and making this submodule private
- Encrypting the input files before pushing them to a public repo

I chose to do the latter using [git-crypt](https://github.com/AGWA/git-crypt). Here's how I did it:

# Git-Crypt
##### Install on macOS with brew:
```sh
brew install git-crypt
```

##### Init and export key:
```sh
cd /path/to/repo
git-crypt init # Generates a random key (in .git/git-crypt/keys/default). It is inaccessible when locked!
git-crypt export-key ../advent_of_code_git-crypt.key # Export key to be able to unlock
```

##### Configure repo
- Create a `.gitattributes` file with the following content:
```
**/input/** filter=git-crypt diff=git-crypt
```
This will encrypt any file within an `input` directory.

##### Helpful commands to test things:
```sh
git-crypt status # check what will be encrypted
git-crypt status -e # show only files that are or should be encrypted
```

The following operations require the working directory to be clean, so **commit** the `.gitattributes` file!

```sh
git-crypt lock # Lock files in repo (happens transparently on push to remote)
git-crypt unlock ../advent_of_code_git-crypt.key # Unlock encrypted files in local repo (also happens transparently on pull)
```

#### Encrypt previously unencrypted files
```sh
git-crypt status -f
```

##### Fix history?
The above leaves unencrypted versions of the files in the git history. To fix this, one would have to rewrite it. This could be done using something like [`git-filter-repo`](https://github.com/newren/git-filter-repo). An older method would be using `git filter-branch`, but it is **not recommended**. That said, the following seemed to work for me (after having made backups!):
```sh
git filter-branch --index-filter 'git rm --cached --ignore-unmatch **/input/day*.txt' HEAD
```

After verifying that the history looked clean, I copied the input files back into the clean repo, to be encrypted on push.