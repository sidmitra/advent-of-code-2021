# Advent of Code 2020

Using the challenge to learn rust!


## Day 1

- Code didn't turn out to be that much larger than a Python equivalent.
- Aleady getting used to last all the static typing. Credit goes to trying out a few of last years exercises in rust, last month.
- Sliding window built-in to stdlib is great. Is it there in python? There's iterools-more for it.


## Day 2
- Used the match statement for the first time, with multiple statements on each pattern. Not even used it yet in Python. Seems neat!

- The tuple unpacking is weird, where you have to call .next() in successive lines, or use a separate itertools crate. Having builtin unpacking of arguments/tuples is a great feature to have built-in like python, and use it all the time.


## Day 3
- Single quote for characters, double for strings. Huh! wasn't expecting that.
- Dereferencing i.e. & etc is slightly easier to grok now.
- List comprehension, filter is a bit weird, and got stuck figuring out the correct type hints.
- Haven't figured out when is something an iter, and when you have to do into_iter(), iter()
- The solution came out fairly ugly with code duplication. Did it just to not have to figure out re-using
  lists/filters just yet.
