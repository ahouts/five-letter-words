# five-letter-words

A solution to the problem posed in
[Can you find: five five-letter words with twenty-five unique letters?](https://www.youtube.com/watch?v=_-AfhLQfb6w)

Single threaded, fairly straightforward implementation that runs in under 30 seconds on my system.
Five nested for loops, completes in a reasonable amount of time because:

1. it uses `continue` if the current word conflicts with the other words we have chosen
2. in each for loop use `i+1` as the starting index, where `i` is the index of the outer for loop

I think this means no duplicate work is done, but I could be wrong.

```
❯ time cargo run --release
    Finished release [optimized] target(s) in 0.00s
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
cargo run --release  27.26s user 0.00s system 99% cpu 27.271 total
```

Word list is taken from [Wordle](https://www.nytimes.com/games/wordle/index.html) as described in the video.

For an apples-to-apples comparison, here it is using the word list from https://github.com/dwyl/english-words.

```
❯ time RUSTFLAGS="-C target-cpu=native" cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `/home/ahouts/Projects/five-letter-words/target/release/five-letter-words`
found 5977 unique sets of 5 letters that appear in words
538 five word anagram groups with no overlapping letters found
bejig hdqrs fconv ampyx klutz
bejig chump vrows fldxt knyaz
bejig quack vrows fldxt nymph
bejig supvr mowch fldxt knyaz
...
vingt frowl pbxes jacky zhmud
vughs wrack fldxt jimpy benzo
vughs wrack fldxt jimpy bonze
supvr whack fldxt benjy gizmo
supvr chawk fldxt benjy gizmo
RUSTFLAGS="-C target-cpu=native" cargo run --release  51.07s user 0.01s system 99% cpu 51.113 total
```