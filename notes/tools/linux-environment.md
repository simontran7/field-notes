# Linux Environment

## Getting Help

- Open the full manual for a command (`q` to quit, `/<pattern>` to search)

```syntax
man <command>
```

- Print a one-line description of a command

```syntax
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
| `!!` | Rerun the previous command |
| `!$` | Insert the last argument of the previous command and rerun |

### Control

| Shortcut | Description |
| :---: | :---: |
| `Tab` | Autocomplete a command, file, or directory name (press twice to list options) |
| `Ctrl+C` | Stop the running command |
| `Ctrl+Z` | Pause the running command and send it to the background |
| `Ctrl+D` | Exit the current shell (same as `exit`) |
| `Ctrl+L` | Clear the screen (same as `clear`) |
| `Ctrl+Shift+C` / `Ctrl+Shift+V` | Copy / paste |

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

```console
$ pwd
```

- Change directory

```syntax
cd <directory>
```

- Go to your home directory

```console
$ cd
```

- Go back to the previous directory

```console
$ cd -
```

- list contents (including dot files) of directory(s) in detailed format

```syntax
ls -al <directory 1> <...> <directory N>
```

- Generate several words from one pattern (brace expansion)

```console
$ mkdir {src,docs,tests}       # src docs tests
$ touch file{1..3}.txt         # file1.txt file2.txt file3.txt
```

### Globbing (Shell Wildcards)

The shell has its own wildcards for matching file names, called **globs**. They look like regex but behave differently.

| Glob | Matches | Example |
| :---: | :---: | :---: |
| `*` | Any number of any characters (including none) | `*.txt` matches every `.txt` file |
| `?` | Exactly one character | `file?.txt` matches `file1.txt` but not `file10.txt` |
| `[abc]` | One character from the set | `file[12].txt` matches `file1.txt` and `file2.txt` |
| `[a-z]` | One character in the range | `[a-c]*` matches names starting with `a`, `b`, or `c` |
| `[!abc]` | One character _not_ in the set | `[!.]*` matches names not starting with `.` |

## Files & Directory Management

- Create a file

```syntax
touch <file>
```

- Creating a directory, while creating missing parents automatically

```syntax
mkdir -p <folder>
```

- Copying files

```syntax
cp <source> <destination>
```

- Copying directories and everything inside them

```syntax
cp -r <source> <destination>
```

- Moving files and directories

```syntax
mv <source> <destination>
```

> [!NOTE]
> Moving a file to the _same_ directory under a different name renames it.

> [!WARNING]
> `mv` silently overwrites `<destination>` if it already exists. Use `mv -i` to be prompted first.

- Removing files

```syntax
rm <file>
```

- Remove an empty directory

```syntax
rmdir <directory>
```

- Remove a directory and everything inside it

```syntax
rm -r <directory>
```

> [!WARNING]
> `rm` deletes permanently, with no trash bin and no undo. Double-check the path before running it, especially with `-r` or wildcards.

- Find files by name, searching recursively from the current directory

```syntax
find . -name '<pattern>'
```

- Create a symbolic link (a shortcut that points to another path)

```syntax
ln -s <target> <link name>
```

- Create a hard link (a second name for the same file data)

```syntax
ln <target> <link name>
```

> [!NOTE]
> A symbolic link breaks if its target is moved or deleted, but it can point to directories and across drives. A hard link keeps working even if the original name is deleted, but it can't point to directories or across drives. Use symbolic links unless you have a specific reason not to.

## I/O Redirection & Piping

Every process has three default streams: **standard input** (stdin, defaults to the keyboard), **standard output** (stdout, defaults to the screen), and **standard error** (stderr, also defaults to the screen). 

- redirect stdout to `file` (overwriting `file` if it exists)

```syntax
<command> > <file>
```

- append stdout to `file`

```syntax
<command> >> <file>
```

- redirect stderr to `file`

```syntax
<command> 2> <file>
```

- redirect both stdout and stderr to `file`

```syntax
<command> &> <file>
```

- discard error messages

```syntax
<command> 2> /dev/null
```

> [!NOTE]
> `/dev/null` is a special file that throws away anything written to it.

- redirect stdin to come from `file` instead of the keyboard

```syntax
<command> < <file>
```

- pipe stdout of `command 1` into stdin of `command 2`

```syntax
<command 1> | <command 2> | <...> | <command N>
```

- show output on the screen _and_ save it to `file`

```syntax
<command> | tee <file>
```

## Regular Expressions (PCRE)

### Character Classes

- Match any single character inside the square brackets

```syntax
[<pattern>]
```

- Match any single character in a range

```syntax
[<start>-<end>]
```

- Match any single character _not_ inside the square brackets

```syntax
[^<pattern>]
```

> [!NOTE]
> Special characters (e.g., `.`, `+`, `*`) lose their meaning when placed in a character class

> [!NOTE]
> Both `-` and `^` are context-sensitive. The `-` only acts as a range operator when it appears between two characters (e.g., `[a-z]`). If it's placed at the very beginning or very end of the brackets (e.g., `[-az]` or `[az-]`), or if it's escaped with a backslash (e.g., `[a\-z]`), regex treats it as a literal hyphen character, not a range. The `^` only negates the entire class when it appears as the first character inside the brackets (e.g., `[^abc]`). If `^` appears anywhere else inside the brackets (e.g., `[a^bc]`), it is treated as a literal caret character and is simply included as one of the characters to match.

#### Shorthand Character Classes

The **lowercase** version matches something, and the **uppercase** version matches the exact opposite (its complement):

| Notation | Description |
| :---: | :---: |
| `\d` | Any digit (same as `[0-9]`) |
| `\D` | Any _non_-digit |
| `\w` | Any alphanumeric (letters, digits) or `_` |
| `\W` | Neither alphanumeric nor `_` |
| `\s` | Any whitespace (space, tab, newline) |
| `\S` | Not whitespace |

### Alternation

- Match the pattern to its left _or_ the pattern to its right

```syntax
<pattern>|<pattern>
```

> [!NOTE]
> Alternation applies to whole subpatterns, so group when needed (e.g., `foo|barbaz` matches `foo` or `barbaz`, while `(foo|bar)baz` matches `foobaz` or `barbaz`)

### Wildcards

- Match any character

```syntax
.
```

- Match `pattern` 0 or 1 times (i.e., optional)

```syntax
(?:<pattern>)?
```

- Match `pattern` 0 or more times

```syntax
(?:<pattern>)*
```

- Match `pattern` 1 or more times

```syntax
(?:<pattern>)+
```

- Match `pattern` exactly $n$ times

```syntax
(?:<pattern>){<n>}
```

- Match `pattern` $n$ or more times

```syntax
(?:<pattern>){<n>,}
```

- Match `pattern` between $n$ and $m$ times

```syntax
(?:<pattern>){<n>,<m>}
```

> [!NOTE]
> A quantifier applies only to the single character, class, or group right before it, so `abc?` makes only the `c` optional. The `(?:<pattern>)` group makes it apply to the whole pattern, and isn't needed for a single character, class, or group (e.g., `a?`, `\d+`, `[a-z]*`).

> [!NOTE]
> To match a wildcard character literally, escape it with a backslash (e.g., `\.`, `\?`, `\*`).

> [!NOTE]
> These are regex wildcards. The shell uses different wildcards for matching file names; see [Globbing](#globbing-shell-wildcards).

> [!NOTE]
> By default, quantifiers are **greedy** (i.e., they match as much as possible). Append `?` to make them **lazy** (i.e., match as little as possible): `*?`, `+?`, `??`, `{n,m}?`.

### Anchors

- Anchor the match to the start of the string (or start of the line in multiline mode)

```syntax
^
```

- Anchor the match to the end of the string (or end of the line in multiline mode)

```syntax
$
```

- Match a word boundary (i.e., the position between a `\w` and a `\W` character, or the start/end of the string)

```syntax
\b
```

- Match a non-word boundary (i.e., any position that `\b` does not match)

```syntax
\B
```

> [!TIP]
> To force the entire string to match the pattern (rather than match a substring somewhere in the middle):
> ```syntax
> ^<pattern>$
> ```

### Groups and Backreferences

- Capturing group (i.e., records what was matched); referenced by position with `\1`, `\2`, … (within the pattern) or `$1`, `$2`, … (in a replacement string)

```syntax
(<pattern>)
```

- Non-capturing group (i.e., groups without recording); preferred when you don't need a backreference, as it keeps group numbering cleaner

```syntax
(?:<pattern>)
```

- Named capturing group; match the same text again later in the pattern with the backreference `\k<name>`

```syntax
(?<<name>><pattern>)
```

> [!NOTE]
> `(\w+) \1` matches a repeated word like `hello hello`, because `\1` must match whatever `(\w+)` captured.

### Lookarounds

> [!DEFINITION]
> **Lookarounds** are zero-width assertions, which means they check for a pattern at the current position without consuming any characters.

- Positive lookahead (i.e., matches if `pattern` follows)

```syntax
(?=<pattern>)
```

- Negative lookahead (i.e., matches if `pattern` does not follow)

```syntax
(?!<pattern>)
```

- Positive lookbehind (i.e., matches if `pattern` precedes)

```syntax
(?<=<pattern>)
```

- Negative lookbehind (i.e., matches if `pattern` does not precede)

```syntax
(?<!<pattern>)
```

> [!NOTE]
> `\d+(?= dollars)` matches the number in `100 dollars` but not in `100 euros`, and the match is just `100`, not `100 dollars`, since the lookahead doesn't consume characters.

> [!WARNING]
> Lookbehind patterns in PCRE must be _fixed-width_ (you can't use `*` or `+` inside them). Lookaheads have no such restriction.

## Text Manipulation

- Print the entire file to the screen

```syntax
cat <file>
```

- Page through the file interactively (`space` for next page, `q` to quit)

```syntax
less <file>
```

- Show the first $n$ lines

```syntax
head -n <n> <file>
```

- Show the last $n$ lines

```syntax
tail -n <n> <file>
```

- Follow a file as new lines are added (useful for logs, `Ctrl+C` to stop)

```syntax
tail -f <file>
```

> [!TIP]
> `less` is used in preference to `cat` for long files, since `cat` dumps the whole file at once, scrolling past the top of the window and leaving it unreadable.

> [!TIP]
> Inside `less`, type `/<pattern>` to search forward for `<pattern>`, then `n` to jump to the next match.

- Search for a pattern recursively

```syntax
grep -rnPI --color=auto --exclude-dir={.git,target} '<pattern>' .
```

- Count lines, words, and characters in a file

```syntax
wc -lwc <file>
```

- Search and replace a single file

```syntax
sed -i 's/<old text>/<new text>/g' <file to search>
```

- Search and replace across several files

```syntax
grep -rIEl '<old text>' . | xargs sed -i 's/<old text>/<new text>/g'
```

> [!WARNING]
> `sed -i` edits files in place with no backup. Run the command without `-i` first to preview the output, or use `-i.bak` to keep a backup copy of each file.

## Permissions & Ownership

```
file type     owner (u)    group (g)   others (o)
    │        ┌────┴────┐  ┌────┴────┐  ┌────┴────┐
    .         .   .   .    .   .   .    .   .   .
              │   │   │
              │   │   └── execute bit
              │   └────── write bit
              └────────── read bit
```

| File Type | Meaning |
| :---: | :---: |
| `-` | Regular file |
| `d` | Directory |
| `l` | Symbolic link |
| `c` | Character device (e.g., a terminal, `/dev/null`) |
| `b` | Block device (e.g., a disk) |
| `p` | Named pipe (FIFO) |
| `s` | Socket |

| Bit | On | Off | On a file | On a directory |
| :---: | :---: | :---: | :---: | :---: |
| Read | `r` | `-` | Read its contents | List the names inside it |
| Write | `w` | `-` | Change its contents | Create, delete, and rename entries inside it |
| Execute | `x` | `-` | Run it as a program | Enter it (`cd`) and access the entries inside it |

### Viewing Permissions

```syntax
ls -l <file>
```

### Changing Permissions

#### Bit Mode

```syntax
chmod <who><operator><permissions> <file>
```

| Who | Meaning |
| :---: | :---: |
| `u` | Owner |
| `g` | Group |
| `o` | Others |
| `a` | Owner, group, and others |

| Operator | Meaning |
| :---: | :---: |
| `+` | Add the permissions |
| `-` | Remove the permissions |
| `=` | Set exactly these permissions, removing any others |

| Permission | Meaning |
| :---: | :---: |
| `r` | Read |
| `w` | Write |
| `x` | Execute |
| `s` | [setuid](#special-bits) with `u`, [setgid](#special-bits) with `g` |
| `t` | [Sticky bit](#special-bits) |

> [!NOTE]
> Combine letters to change several groups and permissions at once: `chmod ug+rw file`.
> Separate different changes with commas: `chmod u+x,go-w script.sh`.

#### Octal Mode

```syntax
chmod <special><owner><group><others> <file>
```

| Digit | Permission |
| :---: | :---: |
| `4` | Read |
| `2` | Write |
| `1` | Execute |

> [!NOTE]
> Add the digits for each group: `7` = `rwx`, `6` = `rw-`, `5` = `r-x`, `4` = `r--`, `0` = `---`.
> e.g. `chmod 755 script.sh` → `rwxr-xr-x`
> The `<special>` digit is optional. See [Special Bits](#special-bits).

### Changing Ownership

```syntax
chown <owner>:<group> <file>
```

> [!NOTE]
> Add `-R` to apply to a directory and everything inside it.

> [!NOTE]
> Omit either side to change only one (but the group must retain the colon).

### Running as Another User

- Run a single command as root

```syntax
sudo <command>
```

- Re-run the previous command with `sudo` (useful after "permission denied")

```console
$ sudo !!
```

- Switch to another user (defaults to root without a user name)

```syntax
su - <user>
```

> [!TIP]
> Prefer `sudo` over `su`. It limits root access to a single command, and it asks for _your_ password rather than the root password.

## Process Management

- List running processes associated with the _current_ shell

```console
$ ps
```

- Display a _full_ listing of _all_ running processes

```console
$ ps -ef # or `ps aux`
```

| Field | Description |
| :---: | :---: |
| UID | The user who owns the process |
| PID | The process ID, a unique identifier assigned to each process |
| PPID | The parent process ID, the process that spawned the current process |
| C |  Average CPU usage over the process's lifetime, as a percentage |
| STIME | The start time for the current process |
| TTY | The controlling terminal for the current process |
| TIME | The amount of CPU time accumulated by the current process |
| CMD | The command used to invoke the process |

- Find a process by name

```syntax
ps -ef | grep <name>
```

- Monitor processes interactively, sorted by CPU usage (`q` to quit)

```console
$ htop
```

> [!NOTE]
> `htop` may need to be installed with `sudo apt install htop`. The older `top` is always available.

- Ask a process to stop (sends `SIGTERM`, letting it clean up first)

```syntax
kill <PID>
```

- Force a process to stop immediately (sends `SIGKILL`)

```syntax
kill -9 <PID>
```

> [!TIP]
> Try plain `kill` first. `kill -9` can't be ignored, but it gives the process no chance to save its work or clean up temporary files.

- Stop processes by name instead of PID

```syntax
pkill <name>
```

### Job Control

- Run a command in the background

```syntax
<command> &
```

- List background and suspended jobs

```console
$ jobs
```

- Bring a job back to the foreground

```syntax
fg %<job number>
```

- Resume a suspended job in the background

```syntax
bg %<job number>
```

## Scheduling Tasks 

Each line in the crontab is one task: five time fields followed by the command to run.

```syntax
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

- Edit your scheduled tasks (opens your crontab in an editor)

```console
$ crontab -e
```

- List your scheduled tasks

```console
$ crontab -l
```

- Delete _all_ your scheduled tasks (without confirmation)

```console
$ crontab -r
```

> [!WARNING]
> Cron runs with a minimal environment, so commands that work in your terminal can fail silently in cron. Use full paths for scripts and files (no `~`), and redirect output to a log file with `>> <file> 2>&1` so you can see what went wrong.

> [!TIP]
> [crontab.guru](https://crontab.guru) translates a schedule like `*/15 9-17 * * 1-5` into plain English, which is handy for checking your timing before saving.

## Shell Scripting & Environment

### Shell Basics

- Show previously run commands

```console
$ history
```

- Show the full path of the program that runs when you type a command

```syntax
which <command>
```

- Exit the current shell (closes the terminal, or ends an SSH session)

```console
$ exit
```

- Log out of a login shell (like an SSH session or a TTY)

```console
$ logout
```

> [!NOTE]
> `logout` only works in a login shell. In a regular terminal window it fails with "not login shell: use `exit`", while `exit` works everywhere.

### Variables

- Set a shell variable

```console
$ NAME="VALUE"
```

> [!WARNING]
> No spaces around `=`. `name = "value"` is treated as running a command called `name`.

- Print a variable

```console
$ echo "$NAME"
```

- Set an environment variable (visible to programs started from this shell)

```console
$ export NAME="VALUE"
```

> [!NOTE]
> A shell (local) variable exists only in the current shell. An exported environment variable is also passed to every program the shell starts. Neither survives closing the terminal; to make one permanent, add it to `~/.bashrc`.

- List all environment variables

```console
$ env
```

- Add a directory to `PATH` (the list of directories searched for commands)

```syntax
export PATH="<directory>:$PATH"
```

### Startup Files

There are two bash startup files:
- `~/.bashrc`: runs every time you open a new terminal. Put aliases, variables, and prompt settings here.
- `~/.bash_profile`: runs once when you log in. On Debian and Ubuntu it also loads `~/.bashrc`.

- Reload `~/.bashrc` after editing it, without opening a new terminal

```console
$ source ~/.bashrc
```

### Aliases

- Create a shortcut for a longer command

```syntax
alias <name>='<command>'
```

- List all aliases

```console
$ alias
```

- Remove an alias

```syntax
unalias <name>
```

> [!NOTE]
> Aliases disappear when the terminal closes. Add the `alias` line to `~/.bashrc` to keep it.

### Exit Codes

> [!DEFINITION]
> Every command returns an **exit code** when it finishes: `0` means success, and anything else means failure.

- Print the exit code of the last command

```console
$ echo $?
```

- Run `command 2` only if `command 1` succeeds

```syntax
<command 1> && <command 2>
```

- Run `command 2` only if `command 1` fails

```syntax
<command 1> || <command 2>
```

- End a script with a specific exit code

```syntax
exit <code>
```

### Shebang (`#!`)

The first line of a script, starting with `#!`, tells the kernel which interpreter should run the file. Running `./hello.sh` with `#!/bin/bash` at the top is equivalent to running `/bin/bash ./hello.sh`.

- Make a script runnable from the console using an interpreter found on your `PATH` (e.g., Bash, Python, Node)

```syntax
#!/usr/bin/env <interpreter>
```

> [!TIP]
> Prefer `#!/usr/bin/env <interpreter>` over a hardcoded path like `#!/usr/bin/python3`. `env` searches your `PATH` for the interpreter, so the script works even when it's installed in a different location on another machine.

> [!TIP]
> Use long form flags in scripts ([source](https://matklad.github.io/2025/03/21/use-long-options-in-scripts.html)).

## Archiving & Compression

- Create a compressed archive

```syntax
tar -czvf <archive>.tar.gz <file or directory>
```

- Extract an archive (into the current directory)

```syntax
tar -xvf <archive>.tar.gz
```

- Extract an archive into a specific directory

```syntax
tar -xvf <archive>.tar.gz -C <directory>
```

- List an archive's contents without extracting

```syntax
tar -tvf <archive>.tar.gz
```

> [!NOTE]
> Flags: `c` create, `x` extract, `t` list, `z` gzip compression, `v` verbose (show each file), `f` archive file name (keep it last, right before the name). When extracting or listing, `tar` detects the compression type automatically, so `-xvf` works for `.tar.gz`, `.tar.xz`, and `.tar.bz2` alike.

- Create a zip archive

```syntax
zip -r <archive>.zip <file or directory>
```

- Extract a zip archive

```syntax
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

```syntax
sudo apt install <package>
```

- Install a local `.deb` file 

```syntax
sudo apt install ./<package>.deb
```

> [!NOTE]
> The `./` matters. Without a path, `apt` looks for a repository package by that name.

- Reinstall a package

```syntax
sudo apt reinstall <package>
```

- Remove a package (keeping its config files and its dependencies)

```syntax
sudo apt remove <package>
```

- Remove a package and its config files (keeping its dependencies)

```syntax
sudo apt purge <package>
```

- Remove orphaned dependencies

```console
$ sudo apt autoremove
```

- Remove orphaned dependencies and their config files

```console
$ sudo apt autoremove --purge
```

- Remove a package, its config files, and orphaned dependencies

```syntax
sudo apt purge --autoremove <package>
```

### Update & Upgrade

- Refresh package lists

```console
$ sudo apt update
```

- Upgrade all packages 

```console
$ sudo apt upgrade
```

- Upgrade all packages (may install or remove packages to resolve dependency conflicts)

```console
$ sudo apt full-upgrade
```

- Upgrade a single package (without installing it if it's missing)

```syntax
sudo apt install --only-upgrade <package>
```

- Refresh and upgrade in one go

```console
$ sudo apt update && sudo apt upgrade -y
```

### Search & Inspect

- Search packages by name and description

```syntax
apt search <keyword>
```

- Show package details (version, dependencies, size, description)

```syntax
apt show <package>
```

- List all installed packages

```console
$ apt list --installed
```

- List packages with pending updates

```console
$ apt list --upgradable
```

- List what a package depends on

```syntax
apt depends <package>
```

- List what depends on a package

```syntax
apt rdepends <package>
```

### Hold Packages

- Prevent a package from being upgraded

```syntax
sudo apt-mark hold <package>
```

- Allow a held package to be upgraded again

```syntax
sudo apt-mark unhold <package>
```

- List held packages

```console
$ apt-mark showhold
```

### Cleanup

- Delete all cached `.deb` files

```console
$ sudo apt clean
```

- Delete only cached `.deb` files that can no longer be downloaded

```console
$ sudo apt autoclean
```

### Fixing Problems

- Fix broken or unmet dependencies

```console
$ sudo apt --fix-broken install
```

- Finish interrupted installs

```console
$ sudo dpkg --configure -a
```

## Users & Groups

- Print the current username

```console
$ whoami
```

- Show your user ID and every group you belong to

```console
$ id
```

- Show who is logged in and what they're doing

```console
$ w
```

- Change the current user's password

```console
$ passwd
```

- Create a new user (prompts for a password and details)

```syntax
sudo adduser <user>
```

- Delete a user

```syntax
sudo deluser <user>
```

- Add a user to a group

```syntax
sudo usermod -aG <group> <user>
```

> [!WARNING]
> Never leave out the `-a`. Without it, `usermod -G` _replaces_ all of the user's groups instead of adding one, which can remove your own `sudo` access.

> [!NOTE]
> Group changes take effect the next time the user logs in.

## System Info & Power

### System Info

- Print kernel and system details

```console
$ uname -a
```

- Show which distribution and version you're running

```console
$ cat /etc/os-release
```

- Print the current date and time

```console
$ date
```

- Show how long the system has been running

```console
$ uptime
```

### Disk & Memory

- Show free space on each drive, in human-readable sizes

```console
$ df -h
```

- Show the total size of a directory

```syntax
du -sh <directory>
```

- Show memory usage

```console
$ free -h
```

- List drives and their partitions

```console
$ lsblk
```

### Power

- Shut down the system immediately

```console
$ sudo shutdown now
```

- Schedule a shutdown

```syntax
sudo shutdown +<n>       # in <n> minutes
sudo shutdown <hh:mm>    # at a specific time
```

- Cancel a scheduled shutdown

```console
$ sudo shutdown -c
```

- Restart the system

```console
$ sudo reboot
```

## Networking

- Show your network interfaces and local IP addresses

```console
$ ip address
```

> [!NOTE]
> `ip address` replaces the older `ifconfig`, which is deprecated and not installed by default on modern Debian and Ubuntu.

- Show your public IP address

```console
$ curl ifconfig.me
```

- Check whether a host is reachable (sends 4 packets)

```syntax
ping -c 4 <host>
```

- Download a file into the current directory

```syntax
curl -O <url>
```

- Show which ports are open and which programs are listening on them

```console
$ sudo ss -tulpn
```

- Connect to a remote machine

```syntax
ssh <user>@<host>
```

- Copy a file to a remote machine

```syntax
scp <file> <user>@<host>:<path>
```

- Copy a file from a remote machine to your local machine

```syntax
scp <user>@<host>:<path/to/file> <local path>
```

- Copy a directory and everything inside it (works in either direction)

```syntax
scp -r <directory> <user>@<host>:<path>
```

> [!TIP]
> Always run `scp` from your _local_ machine. The side with `<user>@<host>:` is the remote one, and the order is always `scp <source> <destination>`, just like `cp`.
