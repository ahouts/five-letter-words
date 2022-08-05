# five-letter-words

A solution to the problem posed in
[Can you find: five five-letter words with twenty-five unique letters?](https://www.youtube.com/watch?v=_-AfhLQfb6w)

Multi-threaded, fairly straightforward implementation that runs in under two seconds on my system.
Five nested for loops, completes in a reasonable amount of time because:

1. it uses `continue` if the current word conflicts with the other words we have chosen
2. in each for loop use `i+1` as the starting index, where `i` is the index of the outer for loop

I think this means no duplicate work is done, but I could be wrong.

```
❯ time RUSTFLAGS="-C target-cpu=native" cargo run --release
    Finished release [optimized] target(s) in 0.01s
     Running `target/release/five-letter-words`
found 5183 unique sets of 5 letters that appear in words
10 five word anagram groups with no overlapping letters found
bling treck waqfs jumpy vozhd
pling treck waqfs jumby vozhd
brick glent waqfs jumpy vozhd
kreng clipt waqfs jumby vozhd
fjord chunk vibex gymps waltz
fjord gucks vibex nymph waltz
prick glent waqfs jumby vozhd
kempt brung waqfs cylix vozhd
kempt brung waqfs xylic vozhd
blunk waqfs cimex grypt vozhd
clunk waqfs bemix grypt vozhd
RUSTFLAGS="-C target-cpu=native" cargo run --release  59.06s user 0.13s system 2981% cpu 1.985 total
```

Word list is taken from [Wordle](https://www.nytimes.com/games/wordle/index.html) as described in the video.

For an apples-to-apples comparison, here it is using the word list from https://github.com/dwyl/english-words.

```
❯ time RUSTFLAGS="-C target-cpu=native" cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/five-letter-words`
found 5977 unique sets of 5 letters that appear in words
538 five word anagram groups with no overlapping letters found
bejig hdqrs fconv ampyx klutz
bejig chump vrows fldxt knyaz
bejig quack vrows fldxt nymph
...
vughs wrack fldxt jimpy benzo
supvr whack fldxt benjy gizmo
supvr chawk fldxt benjy gizmo
RUSTFLAGS="-C target-cpu=native" cargo run --release  113.28s user 0.07s system 2797% cpu 4.051 total
```