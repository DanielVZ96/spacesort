# (S p a c e s o r t)
✨ Fast sorting using space
_Warning: not suitable for production code. Currently in development for learning purposes._

This is a sorting algorithm that mixes comparative and non-comparative methods depending 
on the input's characteristics (length, range of values, min-max values, etc.). It mainly makes 
use of variations of the counting sort algorithm, but also fallbacks to comparison based algorithms 
in cases they are expected to be faster.

# Further Work
- [] Try custom data structure to store sorted elements
- [] More benchmarks for better `should_use_space` function
- [] Better `sort_by` function
- [] Bug: Set sort doesn't support negative values
