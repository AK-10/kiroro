# Kiroro Editor
A toy text Editor.

Rewrote [kilo-editor](https://github.com/antirez/kilo) in Rust.


## amount of codes
```
❯❯❯ cloc ./src
       5 text files.
       5 unique files.
       0 files ignored.

github.com/AlDanial/cloc v 1.90  T=0.01 s (819.9 files/s, 169398.9 lines/s)
-------------------------------------------------------------------------------
Language                     files          blank        comment           code
-------------------------------------------------------------------------------
Rust                             5            119            101            813
-------------------------------------------------------------------------------
SUM:                             5            119            101            813
-------------------------------------------------------------------------------
```

## usage
```
Ctrl-Q: Quit
Ctrl-F: Search
Ctrl-S: Save
```

Search is incremental and able to move next/previous search candidate by arrow key.
- next: right or down key
- previous: left or up key
