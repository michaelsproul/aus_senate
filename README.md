Australian Senate Voting Algorithms
====

[![Build Status](https://travis-ci.org/michaelsproul/aus_senate.svg?branch=master)](https://travis-ci.org/michaelsproul/aus_senate)

This is an implementation of the Australian Senate Voting algorithm as described by the AEC.
The AEC won't show us their code, but we can still verify their results using an independent
implementation!

You can read more about this project on [Medium][medium-article].

All code in [Rust][].

# Running the Code

To download all the CSV files, verify their integrity and run the elections, just do this:

```
$ ./run.py
```

You'll need Python and a Rust compiler.

You can also run elections for a few states of your choice:

```
$ ./run.py NSW SA
```

# Other Implementations

[Grahame Bowland](https://github.com/grahame) maintains a Python implementation
of the same algorithm, and hosts a nice site with the results:

* Code: https://github.com/grahame/dividebatur
* Results: https://angrygoats.net/senate2016/#/index

# License

Copyright Michael Sproul 2016. Licensed under the terms of the [GNU General Public License version 3.0 or later][gpl].

[Rust]: https://www.rust-lang.org
[gpl]: https://www.gnu.org/licenses/gpl-3.0.en.html
[medium-article]: https://medium.com/@michaelsproul/how-to-calculate-a-nation-states-election-result-in-your-bedroom-30f0c5d905af
