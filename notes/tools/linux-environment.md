# Linux Environment

## Getting Help

- Open the full manual for a command (`q` to quit, `/<pattern>` to search)

```bash
man <command>
```

- Print a one-line description of a command

```bash
whatis <command>
```

## Terminal Shortcuts

### Moving the Cursor

| Shortcut | Description |
| :---: | :---: |
| `Ctrl+A` / `Ctrl+E` | Jump to the start / end of the line |
| `Alt+B` / `Alt+F` | Jump back / forward one word |
| `Ctrl+←` / `Ctrl+→` | Jump back / forward one word (in most terminal emulators) |
| `Ctrl+XX` | Toggle between the start of the line and the cursor's current position |

### Editing the Line

| Shortcut | Description |
| :---: | :---: |
| `Ctrl+W` | Delete the word before the cursor |
| `Alt+D` | Delete the word after the cursor |
| `Ctrl+U` / `Ctrl+K` | Delete from the cursor to the start / end of the line |
| `Ctrl+Y` | Paste the text most recently deleted with one of the shortcuts above |
| `Ctrl+_` | Undo the last edit |
| `Alt+T` | Swap the word under the cursor with the one before it |
| `Ctrl+X Ctrl+E` | Open the current command in your text editor, then run it when you save and quit |

### History

| Shortcut | Description |
| :---: | :---: |
| `↑` / `↓` | Scroll through previously run commands |
| `Ctrl+R` | Search command history (press again to cycle through matches) |
| `Alt+.` | Insert the last argument of the previous command (press again to go further back) |
| `!!` | Rerun the previous command (e.g., `sudo !!` reruns it with `sudo`) |
| `!$` | The last argument of the previous command, typed as text |

### Control

| Shortcut | Description |
| :---: | :---: |
| `Tab` | Autocomplete a command, file, or directory name (press twice to list options) |
| `Ctrl+C` | Stop the running command |
| `Ctrl+Z` | Pause the running command and send it to the background (see [Job Control](#job-control)) |
| `Ctrl+D` | Exit the current shell (same as `exit`) |
| `Ctrl+L` | Clear the screen (same as `clear`) |
| `Ctrl+Shift+C` / `Ctrl+Shift+V` | Copy / paste in most terminal emulators |

## Navigation & Filesystem

> [!NOTE]
> If we do not specify a path name to a command, the working directory will be assumed.

> [!NOTE]
> Special path names
> - `/`: root directory (the top of the filesystem)
> - `.`: current directory
> - `..`: parent directory
> - `~`: home directory

- print the name of the current working directory

```bash
pwd
```

- Change directory

```bash
cd <directory>
```

- Go to your home directory

```bash
cd
```

- Go back to the previous directory

```bash
cd -
```

- list contents (including dot files) of directory(s) in detailed format

```bash
ls -al <directory 1> <...> <directory N>
```

- Generate several words from one pattern (brace expansion)

```bash
mkdir {src,docs,tests}       # src docs tests
touch file{1..3}.txt         # file1.txt file2.txt file3.txt
```

### Globbing (Shell Wildcards)

The shell has its own wildcards for matching file names, called **globs**. They look like regex but behave differently.

| Glob | Matches | Example |
| :---: | :---: | :---: |
| `*` | Any number of any characters (including none) | `*.txt` matches every `.txt` file |
| `?` | Exactly one character | `file?.txt` matches `file1.txt` but not `file10.txt` |
| `[abc]` | One character from the set | `file[12].txt` matches `file1.txt` and `file2.txt` |
| `[a-z]` | One character in the range | `[a-c]*` matches names starting with `a`, `b`, or `c` |
| `[!abc]` | One character *not* in the set | `[!.]*` matches names not starting with `.` |

## Files & Directory Management

- Create a file

```bash
touch <file>
```

- Creating a directory, while creating missing parents automatically

```bash
mkdir -p <folder>
```

- Copying files

```bash
cp <source> <destination>
```

- Copying directories and everything inside them

```bash
cp -r <source> <destination>
```

- Moving files and directories

```bash
mv <source> <destination>
```

> [!NOTE]
> Moving a file to the *same* directory under a different name renames it.

> [!CAUTION]
> `mv` silently overwrites `<destination>` if it already exists. Use `mv -i` to be prompted first.

- Removing files

```bash
rm <file>
```

- Remove an empty directory

```bash
rmdir <directory>
```

- Remove a directory and everything inside it

```bash
rm -r <directory>
```

> [!CAUTION]
> `rm` deletes permanently, with no trash bin and no undo. Double-check the path before running it, especially with `-r` or wildcards.

- Find files by name, searching recursively from the current directory

```bash
find . -name '<pattern>'
```

- Create a symbolic link (a shortcut that points to another path)

```bash
ln -s <target> <link name>
```

- Create a hard link (a second name for the same file data)

```bash
ln <target> <link name>
```

> [!NOTE]
> A symbolic link breaks if its target is moved or deleted, but it can point to directories and across drives. A hard link keeps working even if the original name is deleted, but it can't point to directories or across drives. Use symbolic links unless you have a specific reason not to.

## I/O Redirection & Piping

Every process has three default streams: **standard input** (stdin, defaults to the keyboard), **standard output** (stdout, defaults to the screen), and **standard error** (stderr, also defaults to the screen). 

- redirect stdout to `file` (overwriting `file` if it exists)

```bash
<command> > <file>
```

- append stdout to `file`

```bash
<command> >> <file>
```

- redirect stderr to `file`

```bash
<command> 2> <file>
```

- redirect both stdout and stderr to `file`

```bash
<command> &> <file>
```

- discard error messages

```bash
<command> 2> /dev/null
```

> [!NOTE]
> `/dev/null` is a special file that throws away anything written to it.

- redirect stdin to come from `file` instead of the keyboard

```bash
<command> < <file>
```

- pipe stdout of `command 1` into stdin of `command 2`

```bash
<command 1> | <command 2> | <...> | <command N>
```

- show output on the screen *and* save it to `file`

```bash
<command> | tee <file>
```

## Regular Expressions (PCRE)

### Character Classes

- `[<pattern>]`: matches any single character inside the enclosing square brackets.
- `[<start>-<end>]`: range
- `[^<pattern>]`: negates the list

> [!NOTE]
> Special characters (e.g., `.`, `+`, `*`) lose their meaning when placed in a character class

> [!NOTE]
> Both `-` and `^` are context-sensitive. The `-` only acts as a range operator when it appears between two characters (e.g., `[a-z]`). If it's placed at the very beginning or very end of the brackets (e.g., `[-az]` or `[az-]`), or if it's escaped with a backslash (e.g., `[a\-z]`), regex treats it as a literal hyphen character, not a range. The `^` only negates the entire class when it appears as the first character inside the brackets (e.g., `[^abc]`). If `^` appears anywhere else inside the brackets (e.g., `[a^bc]`), it is treated as a literal caret character and is simply included as one of the characters to match.

#### Shorthand Character Classes

The **lowercase** version matches something, and the **uppercase** version matches the exact opposite (its complement):

| Notation | Description |
| :---: | :---: |
| `\d` | Any digit (same as `[0-9]`) |
| `\D` | Any *non*-digit |
| `\w` | Any alphanumeric (letters, digits) or `_` |
| `\W` | Neither alphanumeric nor `_` |
| `\s` | Any whitespace (space, tab, newline) |
| `\S` | Not whitespace |

### Alternation

`<pattern>|<pattern>`: matches the pattern to its left *or* the pattern to its right

> [!NOTE]
> Alternation applies to whole subpatterns, so group when needed (e.g., `foo|barbaz` matches `foo` or `barbaz`, while `(foo|bar)baz` matches `foobaz` or `barbaz`)

### Wildcards

| Notation | Description                            |
| :---: | :---: |
| `.`      | Matches to any character               |
| `?`      | Optional previous (i.e., 0 or 1 of previous) |
| `*`      | 0 or more of previous                  |
| `+`      | 1 or more of previous                  |
| `{n}`    | Exactly `n` of previous                |
| `{n,}`   | `n` or more of previous                |
| `{n, m}` | Between `n` and `m` times of previous  |

> [!NOTE]
> To match it literally, escape it (i.e., `\<wildcard>`).

> [!NOTE]
> These are regex wildcards. The shell uses different wildcards for matching file names; see [Globbing](#globbing-shell-wildcards).

> [!NOTE]
> By default, quantifiers are **greedy** (i.e., they match as much as possible). Append `?` to make them **lazy** (i.e., match as little as possible): `*?`, `+?`, `??`, `{n, m}?`.

### Anchors

- `^`: anchors the match to the start of the string (or start of the line in multiline mode)
- `$`: anchors the match to the end of the string (or end of the line in multiline mode)
- `\b`: word boundary (i.e., matches the position between a `\w` and a `\W` character (or the start/end of the string))
- `\B`: non-word boundary (i.e., matches any position that `\b` does not)

> [!NOTE]
> Using `^<pattern>$` forces the entire string to match the pattern, instead of letting the regex match a substring somewhere in the middle.

### Groups and Backreferences

- `(<pattern>)`: capturing group (i.e., records what was matched); referenced by position with `\1`, `\2`, … (within the pattern) or `$1`, `$2`, … (in a replacement string)
- `(?:<pattern>)`: non-capturing group (i.e., groups without recording); preferred when you don't need a backreference, as it keeps group numbering cleaner
- `(?<name><pattern>)`: named capturing group; referenced with `\k<name>` within the pattern

> [!NOTE]
> `(\w+) \1` matches a repeated word like `hello hello`, because `\1` must match whatever `(\w+)` captured.

### Lookarounds

> [!DEFINITION]
> **Lookarounds** are zero-width assertions, which means they check for a pattern at the current position without consuming any characters.

- `(?=<pattern>)`: positive lookahead (i.e., matches if `pattern` follows)
- `(?!<pattern>)`: negative lookahead (i.e., matches if `pattern` does not follow)
- `(?<=<pattern>)`: positive lookbehind (i.e., matches if `pattern` precedes)
- `(?<!<pattern>)`: negative lookbehind (i.e., matches if `pattern` does not precede)

> [!NOTE]
> `\d+(?= dollars)` matches the number in `100 dollars` but not in `100 euros`, and the match is just `100`, not `100 dollars`, since the lookahead doesn't consume characters.

> [!IMPORTANT]
> Lookbehind patterns in PCRE must be *fixed-width* (you can't use `*` or `+` inside them). Lookaheads have no such restriction.

## Text Manipulation

- Print the entire file to the screen

```bash
cat <file>
```

- Page through the file interactively (`space` for next page, `q` to quit)

```bash
less <file>
```

- Show the first \\(n\\) lines

```bash
head -n <n> <file>
```

- Show the last \\(n\\) lines

```bash
tail -n <n> <file>
```

- Follow a file as new lines are added (useful for logs, `Ctrl+C` to stop)

```bash
tail -f <file>
```

> [!TIP]
> `less` is used in preference to `cat` for long files, since `cat` dumps the whole file at once, scrolling past the top of the window and leaving it unreadable.

> [!TIP]
> Inside `less`, type `/<pattern>` to search forward for `<pattern>`, then `n` to jump to the next match.

- Search for a pattern recursively

```bash
grep -rnEI --color=auto --exclude-dir={.git,target} '<pattern>' .
```

- Count lines, words, and characters in a file

```bash
wc -lwc <file>
```

- Search and replace a single file

```bash
sed -i 's/<old text>/<new text>/g' <file to search>
```

- Search and replace across several files

```bash
grep -rIEl '<old text>' . | xargs sed -i 's/<old text>/<new text>/g'
```

> [!CAUTION]
> `sed -i` edits files in place with no backup. Run the command without `-i` first to preview the output, or use `-i.bak` to keep a backup copy of each file.

## Permissions & Ownership

Every file has an **owner**, a **group**, and three sets of permissions: one for the owner (`u`), one for the group (`g`), and one for everyone else (`o`). Each set is made of **read** (`r`), **write** (`w`), and **execute** (`x`).

For example, `-rwxr-xr--` breaks down as:

| Part | Meaning |
| :---: | :---: |
| `-` | File type (`-` for a file, `d` for a directory, `l` for a symbolic link) |
| `rwx` | Owner can read, write, and execute |
| `r-x` | Group can read and execute |
| `r--` | Everyone else can only read |

- View permissions, owner, and group

```bash
ls -l <file>
```

- Make a file executable

```bash
chmod +x <file>
```

- Add or remove permissions for specific sets (e.g., `u+w`, `g-w`, `o-r`)

```bash
chmod <who><+ or -><permissions> <file>
```

- Set permissions with numbers (read = 4, write = 2, execute = 1, added together per set)

```bash
chmod 644 <file>        # rw-r--r--: typical for regular files
chmod 755 <file>        # rwxr-xr-x: typical for scripts and directories
```

> [!CAUTION]
> Avoid `chmod 777`, which lets *every* user read, modify, and run the file. It's a common quick fix in old tutorials, but it's almost never the right one and creates a security hole.

- Change a file's owner and group (add `-R` to apply to a directory's contents)

```bash
sudo chown <user>:<group> <file>
```

- Run a single command as root

```bash
sudo <command>
```

- Re-run the previous command with `sudo` (useful after "permission denied")

```bash
sudo !!
```

- Switch to another user (defaults to root without a user name)

```bash
su - <user>
```

> [!TIP]
> Prefer `sudo` over `su`. It limits root access to a single command, and it asks for *your* password rather than the root password.

### Special Bits

| Bit | Number | Effect |
| :---: | :---: | :---: |
| setuid | `4` | Executable runs as the file's owner instead of the user who ran it (e.g., `passwd`) |
| setgid | `2` | On a directory, new files inherit the directory's group |
| sticky | `1` | On a directory, only a file's owner can delete it (e.g., `/tmp`) |

> [!NOTE]
> Special bits go in front of the regular numbers, so `chmod 1777 <directory>` sets the sticky bit plus `rwxrwxrwx`. In `ls -l` output they show up as `s` or `t` in place of `x`.

## Process Management

- List running processes associated with the _current_ shell

```bash
ps
```

- Display a _full_ listing of _all_ running processes

```bash
ps -ef # or `ps aux`
```

| Field | Description |
| :---: | :---: |
| UID | The user who owns the process |
| PID | The process ID, a unique identifier assigned to each process |
| PPID | The parent process ID, the process that spawned the current process |
| C | This field is obsolete |
| STIME | The start time for the current process |
| TTY | The controlling terminal for the current process |
| TIME | The amount of CPU time accumulated by the current process |
| CMD | The command used to invoke the process |

- Find a process by name

```bash
ps -ef | grep <name>
```

- Monitor processes interactively, sorted by CPU usage (`q` to quit)

```bash
htop
```

> [!NOTE]
> `htop` may need to be installed with `sudo apt install htop`. The older `top` is always available.

- Ask a process to stop (sends `SIGTERM`, letting it clean up first)

```bash
kill <PID>
```

- Force a process to stop immediately (sends `SIGKILL`)

```bash
kill -9 <PID>
```

> [!TIP]
> Try plain `kill` first. `kill -9` can't be ignored, but it gives the process no chance to save its work or clean up temporary files.

- Stop processes by name instead of PID

```bash
pkill <name>
```

### Job Control

- Run a command in the background

```bash
<command> &
```

- Suspend the foreground command

`Ctrl+Z`

- List background and suspended jobs

```bash
jobs
```

- Bring a job back to the foreground

```bash
fg %<job number>
```

- Resume a suspended job in the background

```bash
bg %<job number>
```

## Scheduling Tasks 

- Edit your scheduled tasks (opens your crontab in an editor)

```bash
crontab -e
```

> [!NOTE]
> The first time you run `crontab -e`, it asks which editor to use.

- List your scheduled tasks

```bash
crontab -l
```

- Delete *all* your scheduled tasks

```bash
crontab -r
```

> [!CAUTION]
> `crontab -r` deletes everything immediately, with no confirmation. The `r` key sits right next to `e`, so it's easy to hit by accident. Back up first with `crontab -l > crontab-backup.txt`.

### Crontab Syntax

Each line in the crontab is one task: five time fields followed by the command to run.

```text
* * * * * <command>
│ │ │ │ │
│ │ │ │ └── day of week (0–7, Sunday is 0 or 7)
│ │ │ └──── month (1–12)
│ │ └────── day of month (1–31)
│ └──────── hour (0–23)
└────────── minute (0–59)
```

| Notation | Meaning | Example |
| :---: | :---: | :---: |
| `*` | Every value | `* * * * *` runs every minute |
| `<n>` | A specific value | `30 9 * * *` runs at 9:30 every day |
| `*/<n>` | Every `n` units | `*/15 * * * *` runs every 15 minutes |
| `<a>,<b>` | A list of values | `0 9,17 * * *` runs at 9:00 and 17:00 |
| `<a>-<b>` | A range of values | `0 9 * * 1-5` runs at 9:00 on weekdays |

| Shortcut | Meaning |
| :---: | :---: |
| `@reboot` | Once, every time the system starts |
| `@hourly` | Same as `0 * * * *` |
| `@daily` | Same as `0 0 * * *` |
| `@weekly` | Same as `0 0 * * 0` |

- Example: run a backup script every day at 2:00 AM, saving its output to a log

```text
0 2 * * * /home/<user>/scripts/backup.sh >> /home/<user>/backup.log 2>&1
```

> [!WARNING]
> Cron runs with a minimal environment, so commands that work in your terminal can fail silently in cron. Use full paths for scripts and files (no `~`), and redirect output to a log file with `>> <file> 2>&1` so you can see what went wrong.

> [!TIP]
> [crontab.guru](https://crontab.guru) translates a schedule like `*/15 9-17 * * 1-5` into plain English, which is handy for checking your timing before saving.

## Shell Scripting & Environment

### Shell Basics

- Show previously run commands

```bash
history
```

- Show the full path of the program that runs when you type a command

```bash
which <command>
```

- Exit the current shell (closes the terminal, or ends an SSH session)

```bash
exit
```

- Log out of a login shell (like an SSH session or a TTY)

```bash
logout
```

> [!NOTE]
> `logout` only works in a login shell. In a regular terminal window it fails with "not login shell: use `exit`", while `exit` works everywhere.

### Variables

- Set a shell variable

```bash
NAME="VALUE"
```

> [!IMPORTANT]
> No spaces around `=`. `name = "value"` is treated as running a command called `name`.

- Print a variable

```bash
echo "$NAME"
```

- Set an environment variable (visible to programs started from this shell)

```bash
export NAME="VALUE"
```

> [!NOTE]
> A shell (local) variable exists only in the current shell. An exported environment variable is also passed to every program the shell starts. Neither survives closing the terminal; to make one permanent, add it to `~/.bashrc`.

- List all environment variables

```bash
env
```

- Add a directory to `PATH` (the list of directories searched for commands)

```bash
export PATH="<directory>:$PATH"
```

### Startup Files

- `~/.bashrc`: runs every time you open a new terminal. Put aliases, variables, and prompt settings here.
- `~/.profile` (or `~/.bash_profile`): runs once when you log in. On Debian and Ubuntu it also loads `~/.bashrc`.

- Reload `~/.bashrc` after editing it, without opening a new terminal

```bash
source ~/.bashrc
```

### Aliases

- Create a shortcut for a longer command

```bash
alias <name>='<command>'
```

- List all aliases

```bash
alias
```

- Remove an alias

```bash
unalias <name>
```

> [!NOTE]
> Aliases disappear when the terminal closes. Add the `alias` line to `~/.bashrc` to keep it.

### Exit Codes

Every command returns an **exit code** when it finishes: `0` means success, and anything else means failure.

- Print the exit code of the last command

```bash
echo $?
```

- Run `command 2` only if `command 1` succeeds

```bash
<command 1> && <command 2>
```

- Run `command 2` only if `command 1` fails

```bash
<command 1> || <command 2>
```

- End a script with a specific exit code

```bash
exit <code>
```

### Shebang (`#!`)

The first line of a script, starting with `#!`, tells the kernel which interpreter should run the file. Running `./hello.sh` with `#!/bin/bash` at the top is equivalent to running `/bin/bash ./hello.sh`.

- Run a script with Bash

```bash
#!/usr/bin/env bash

echo "Hello World!"
```

- Run a script with an interpreter found on your `PATH` (e.g., Bash, Python, Node)

```bash
#!/usr/bin/env <interpreter>
```

> [!TIP]
> Prefer `#!/usr/bin/env <interpreter>` over a hardcoded path like `#!/usr/bin/python3`. `env` searches your `PATH` for the interpreter, so the script works even when it's installed in a different location on another machine.

- Make a script executable, then run it

```bash
chmod +x <script>
./<script>
```

> [!NOTE]
> The `./` is required. The shell only searches `PATH` for commands, not the current directory, so `<script>` alone gives "command not found."

> [!TIP]
> Use long form flags in scripts ([source](https://matklad.github.io/2025/03/21/use-long-options-in-scripts.html)).

## Archiving & Compression

- Create a compressed archive

```bash
tar -czvf <archive>.tar.gz <file or directory>
```

- Extract an archive (into the current directory)

```bash
tar -xvf <archive>.tar.gz
```

- Extract an archive into a specific directory

```bash
tar -xvf <archive>.tar.gz -C <directory>
```

- List an archive's contents without extracting

```bash
tar -tvf <archive>.tar.gz
```

> [!NOTE]
> Flags: `c` create, `x` extract, `t` list, `z` gzip compression, `v` verbose (show each file), `f` archive file name (keep it last, right before the name). When extracting or listing, `tar` detects the compression type automatically, so `-xvf` works for `.tar.gz`, `.tar.xz`, and `.tar.bz2` alike.

- Create a zip archive

```bash
zip -r <archive>.zip <file or directory>
```

- Extract a zip archive

```bash
unzip <archive>.zip
```

> [!NOTE]
> `zip` and `unzip` may need to be installed with `sudo apt install zip unzip`.

## Package Management (Debian/Ubuntu)

> [!TIP]
> Use `apt` for interactive use and `apt-get` in scripts, since its output format is guaranteed stable.

> [!NOTE]
> All commands that modify the system need `sudo` unless you're root.

### Install & Remove

- Install a package

```bash
sudo apt install <package>
```

- Install a local `.deb` file 

```bash
sudo apt install ./<package>.deb
```

> [!NOTE]
> The `./` matters. Without a path, `apt` looks for a repository package by that name.

- Reinstall a package

```bash
sudo apt reinstall <package>
```

- Remove a package (keeping its config files and its dependencies)

```bash
sudo apt remove <package>
```

- Remove a package and its config files (keeping its dependencies)

```bash
sudo apt purge <package>
```

- Remove orphaned dependencies

```bash
sudo apt autoremove
```

- Remove orphaned dependencies and their config files

```bash
sudo apt autoremove --purge
```

- Remove a package, its config files, and orphaned dependencies

```bash
sudo apt purge --autoremove <package>
```

### Update & Upgrade

- Refresh package lists

```bash
sudo apt update
```

- Upgrade all packages 

```bash
sudo apt upgrade
```

- Upgrade all packages (may install or remove packages to resolve dependency conflicts)

```bash
sudo apt full-upgrade
```

- Upgrade a single package (without installing it if it's missing)

```bash
sudo apt install --only-upgrade <package>
```

- Refresh and upgrade in one go

```bash
sudo apt update && sudo apt upgrade -y
```

### Search & Inspect

- Search packages by name and description

```bash
apt search <keyword>
```

- Show package details (version, dependencies, size, description)

```bash
apt show <package>
```

- List all installed packages

```bash
apt list --installed
```

- List packages with pending updates

```bash
apt list --upgradable
```

- List what a package depends on

```bash
apt depends <package>
```

- List what depends on a package

```bash
apt rdepends <package>
```

### Hold Packages

- Prevent a package from being upgraded

```bash
sudo apt-mark hold <package>
```

- Allow a held package to be upgraded again

```bash
sudo apt-mark unhold <package>
```

- List held packages

```bash
apt-mark showhold
```

### Cleanup

- Delete all cached `.deb` files

```bash
sudo apt clean
```

- Delete only cached `.deb` files that can no longer be downloaded

```bash
sudo apt autoclean
```

### Fixing Problems

- Fix broken or unmet dependencies

```bash
sudo apt --fix-broken install
```

- Finish interrupted installs

```bash
sudo dpkg --configure -a
```

## Users & Groups

- Print the current username

```bash
whoami
```

- Show your user ID and every group you belong to

```bash
id
```

- Show who is logged in and what they're doing

```bash
w
```

- Change the current user's password

```bash
passwd
```

- Create a new user (prompts for a password and details)

```bash
sudo adduser <user>
```

- Delete a user

```bash
sudo deluser <user>
```

- Add a user to a group

```bash
sudo usermod -aG <group> <user>
```

> [!CAUTION]
> Never leave out the `-a`. Without it, `usermod -G` *replaces* all of the user's groups instead of adding one, which can remove your own `sudo` access.

> [!NOTE]
> Group changes take effect the next time the user logs in.

> [!NOTE]
> `/etc/passwd` lists every user account and is readable by everyone. Despite its name, it doesn't hold passwords. The password hashes live in `/etc/shadow`, which only root can read.

## System Info & Power

### System Info

- Print kernel and system details

```bash
uname -a
```

- Show which distribution and version you're running

```bash
cat /etc/os-release
```

- Print the current date and time

```bash
date
```

- Show how long the system has been running

```bash
uptime
```

### Disk & Memory

- Show free space on each drive, in human-readable sizes

```bash
df -h
```

- Show the total size of a directory

```bash
du -sh <directory>
```

- Show memory usage

```bash
free -h
```

- List drives and their partitions

```bash
lsblk
```

### Power

- Shut down the system immediately

```bash
sudo shutdown now
```

- Schedule a shutdown

```bash
sudo shutdown +<n>       # in <n> minutes
sudo shutdown <hh:mm>    # at a specific time
```

- Cancel a scheduled shutdown

```bash
sudo shutdown -c
```

- Restart the system

```bash
sudo reboot
```

## Networking

- Show your network interfaces and local IP addresses

```bash
ip address
```

> [!NOTE]
> `ip address` replaces the older `ifconfig`, which is deprecated and not installed by default on modern Debian and Ubuntu.

- Show your public IP address

```bash
curl ifconfig.me
```

- Check whether a host is reachable (sends 4 packets)

```bash
ping -c 4 <host>
```

- Download a file into the current directory

```bash
curl -O <url>
```

- Show which ports are open and which programs are listening on them

```bash
sudo ss -tulpn
```

- Connect to a remote machine

```bash
ssh <user>@<host>
```

- Copy a file to a remote machine

```bash
scp <file> <user>@<host>:<path>
```

- Copy a file from a remote machine to your local machine

```bash
scp <user>@<host>:<path/to/file> <local path>
```

- Copy a directory and everything inside it (works in either direction)

```bash
scp -r <directory> <user>@<host>:<path>
```

> [!TIP]
> Always run `scp` from your *local* machine. The side with `<user>@<host>:` is the remote one, and the order is always `scp <source> <destination>`, just like `cp`.
