# Linux & Git Commands Cheat Sheet

## Linux Commands 🐧

### File & Directory Management
* `pwd`: **P**rint **w**orking **d**irectory. Shows your current location.
* `ls`: **L**i**s**t files and directories. Add `-l` for a long format, and `-a` to show all files including hidden ones.
* `cd <directory>`: **C**hange **d**irectory.
* `mkdir <directory_name>`: **M**a**k**e a new **dir**ectory.
* `rm <file_name>`: **R**e**m**ove a file. Use `-r` for directories.
* `cp <source> <destination>`: **C**o**p**y files or directories.
* `mv <source> <destination>`: **M**o**v**e or rename files and directories.
* `touch <file_name>`: Creates a new, empty file.

### Viewing & Editing Files
* `cat <file_name>`: Con**cat**enates and displays the content of a file.
* `less <file_name>`: Views file content one page at a time.
* `head <file_name>`: Displays the **head** (first 10 lines) of a file.
* `tail <file_name>`: Displays the **tail** (last 10 lines) of a file.
* `nano <file_name>`: Opens a file in the **nano** text editor.
* `vim <file_name>`: Opens a file in the **Vim** text editor.

### System Information
* `uname -a`: **U**nix **name**; shows system info.
* `whoami`: Displays the current user.
* `df -h`: **D**isk **f**ree; shows disk usage. `-h` is for human-readable format.
* `du -sh`: **D**isk **u**sage; shows file or directory size.

### User & Permissions
* `sudo <command>`: **S**uper**u**ser **do**; executes a command with root privileges.
* `chown <user> <file>`: **Ch**ange **own**er of a file.
* `chmod <permissions> <file>`: **Ch**ange file **mod**e (permissions).

---

## Git Commands 🐙

### Basic Workflow
* `git init`: **Init**ializes a new Git repository.
* `git clone <url>`: **Clone**s a remote repository.
* `git add <file>`: **Add**s a file to the staging area. Use `.` to add all files.
* `git commit -m "message"`: **Commit**s staged changes with a message.
* `git status`: Shows the **status** of your working tree.
* `git log`: Shows the **log** of commits.

### Branches & Remotes
* `git branch`: Lists all local **branch**es.
* `git branch <branch_name>`: Creates a new branch.
* `git checkout <branch_name>`: Switches to a different branch.
* `git merge <branch_name>`: **Merge**s another branch into your current branch.
* `git remote -v`: Lists remote repositories.
* `git push <remote> <branch>`: **Push**es local commits to a remote repository.
* `git pull <remote> <branch>`: **Pull**s changes from a remote repository.

### Reverting & Undoing
* `git reset <file>`: **Reset**s a file from the staging area.
* `git checkout -- <file>`: Discards changes in a file.
* `git revert <commit_hash>`: **Revert**s a specific commit by creating a new one.
