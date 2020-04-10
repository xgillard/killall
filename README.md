# killall
This is basically what I wish the killall utility was, and because I could not
find its sources to install on a system where I needed it. So I decided to
write it for myself and came along with this.

If you like it, thats a bonus.

```
killall 0.1.0
Killall Lets you either kill all processes matching a given regex (optionally filtering if the process belongs to some
predefined user).

Or it can kill a whole tree of processes descending from some root process pid. (again with the possibility of filtering
on the owner of the processes).

The default owner for all process is always the current user.

# Note: This is basically what I wish the killall utility was, and because I could not find its sources to install on a
system where I needed it. So I decided to write it for myself and came along with this.

USAGE:
    killall <SUBCOMMAND>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


SUBCOMMANDS:
    children-of    This subcommand lets you kill all the processes that were spawned by the `pid` process, or any of
                   its descendants
    help           Prints this message or the help of the given subcommand(s)
    matching       This subcommand lets you kill all the processes that match a given pattern. Note: Because killall
                   is basically a shim around ps, you might need to be a little bit careful with your 'name'.
                   Indeed, the filtering is done as if it were done by `ps -alx | grep $pattern`
```
