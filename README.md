**JOP**

=>'java top'

* Simple commandline utility that parses the output of java verbose gc and plots it in a graph
* Can be used for tailing the file, or on historic files
* Sample usage:

```bash
java -verbose:gc -cp src Filler >outfile
```
and: 
```bash
jop outfile
```
or:
```bash
cargo run outfile
```

* tested on jdk20/G1 collector
* depends on tail command, so works best on linux/unix/macos