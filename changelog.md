# Uiua Changelog

Uiua is not yet stable.

## 0.0.18 - 2023-10-??
### Language
- [`if` `?`](https://uiua.org/docs/if)'s branches can now have a different number of arguments (but not outputs)
- [`under` `⍜`](https://uiua.org/docs/under) [`&n`](https://uiua.org/docs/&n) can be used to time things
### Interpreter
- The formatter now aligns consecutive end-of-line comments
- Fix a bug that prevent [`under` `⍜`](https://uiua.org/docs/under) multidimensional [`take` `↙`](https://uiua.org/docs/take) and [`drop` `↘`](https://uiua.org/docs/drop) from working

## 0.0.17 - 2023-10-07
### Language
- Add GIF encoding with [`&gife`](https://uiua.org/docs/&gife)
- Rename `constant` to [`box` `□`](https://uiua.org/docs/box).
- Add [`unbox` `⊔`](https://uiua.org/docs/unbox), which unboxes a boxed array
- **Major Change:** Some uses of [`call` `!`](https://uiua.org/docs/call) will no longer compile without declaring a stack signature. When unboxing [`box` `□`](https://uiua.org/docs/box)ed arrays, you can use [`unbox` `⊔`](https://uiua.org/docs/unbox) instead, which has a well-defined signature.
- Add [`fall` `⍖`](https://uiua.org/docs/fall) function, which gives the indices of the array if it were sorted descending
- Change `grade` `⌂` name and glyph to [`rise` `⍏`](https://uiua.org/docs/rise) to reflect its relationship with [`fall` `⍖`](https://uiua.org/docs/fall). Code using `⌂` will continue to work and will be formatted as `⍏`.
- [`try` `⍣`](https://uiua.org/docs/try) now puts arguments to its first function *above* the error value when calling the error handler
- [`fold` `∧`](https://uiua.org/docs/fold) can now use multiple accumulators
- Improve [`dump`](https://uiua.org/docs/dump) output formatting
- [`dump`](https://uiua.org/docs/dump) is now a monadic modifier. Its function preprocesses each value before dumping it.
- Add the [`sig`](https://uiua.org/docs/sig) function, which returns the stack signature of a value
- A negative dimensions in the shape passed to [`reshape` `↯`](https://uiua.org/docs/reshape) can now be in *any* position, not just the first or last
- Functions with ASCII glyphs now also format from their names
- Add a advice diagnostic about the captialization of binding names
### Interpreter
- A few performance improvements, particularly to [`keep` `▽`](https://uiua.org/docs/keep), [`fork` `⊃`](https://uiua.org/docs/fork), and [`under` `⍜`](https://uiua.org/docs/under)
### Website
- Add GIF output
- Execution time limit is now 2 seconds by default but can be customized

## 0.0.16 - 2023-10-05
### Interpreter
- Fix a crash and a bug that could occur when creating nested arrays that pull in values.

## 0.0.15 - 2023-10-05
This version changes a lot of glyphs. If you are coming from the previous version, most of the old glyphs will be automatically formatted to the new ones. The only change you may need to make is replacing all `^`s with `|`s.

You may want to read the new version of the [Advanced Stack Manipulation Tutorial](https://uiua.org/docs/advancedstack) to understand the reason for so many of these changes.

### Language
- Add the [`bracket` `⊓`](https://uiua.org/docs/bracket) modifier, which calls two functions each on different arguments
- Change [`fill` `⬚`](https://uiua.org/docs/fill)'s glyph to reflect its relationship with [`box` `□`](https://uiua.org/docs/box). Code using `⍛` with continue to work and will be formatted as `⬚`.
- Change `share` `⇉` name and glyph to [`fork` `⊃`](https://uiua.org/docs/fork). Code using `⇉` will continue to work and will be formatted as `⊃`.
- Change `noop` `·` name and glyphs to [`identity` `∘`](https://uiua.org/docs/identity) to reflect its relationship with [`gap` `⋅`](https://uiua.org/docs/gap) and [`dip` `⊙`](https://uiua.org/docs/dip). Code using `·` will continue to work and will be formatted as `∘`.
- Change [`identity` `∘`](https://uiua.org/docs/identity)'s signature from `|0.0` to `|1.1`
- Add the [`gap` `⋅`](https://uiua.org/docs/gap) modifier, which discards a value then calls its function. It is mainly intended to be used with [`fork` `⊃`](https://uiua.org/docs/fork).
- Change [`dip` `⊙`](https://uiua.org/docs/dip)'s glyph to reflect its relationship with [`gap` `⋅`](https://uiua.org/docs/gap) and [`identity` `∘`](https://uiua.org/docs/identity). Code using `→` will continue to work and will be formatted as `⊙`.
- Change [`both` `∩`](https://uiua.org/docs/both)'s glyph to reflect its relationship with [`fork` `⊃`](https://uiua.org/docs/fork). Code using `∷` will continue to work and will be formatted as `∩`.
- [`distribute` `∺`](https://uiua.org/docs/distribute) now works with any number of arguments. Only the first argument is distributed.
- [`fill` `⬚`](https://uiua.org/docs/fill) now works with [`reshape` `↯`](https://uiua.org/docs/reshape)
- [`reshape` `↯`](https://uiua.org/docs/reshape) now allow negative numbers to denote derived dimensions
- Change the modifier termination character to `|` instead of `^`
- Remove old versions of `fork` and `trident`
- Add the [`&httpsw`](https://uiua.org/docs/&httpsw) function for making HTTPS requests
### Interpreter
- Add formatter configuration options. See the [readme](https://github.com/uiua-lang/uiua#formatter-configuration) for details.
- Checking for updates is less zealous, and can be disabled with the `--no-update` flag to `uiua run` or `uiua watch`
### Website
- Running code in the Pad editor updates the URL to prevent work from accidentally being lost

# Pre-Version Updates

## 2023-10-03
### Language
- Add the [`share` `⊃`](https://uiua.org/docs/fork) modifier, which unifies and deprecates [`fork` `⊃`](https://uiua.org/docs/fork) and [`trident` `∋`](https://uiua.org/docs/trident)
- [`bind` `'`] no longer calls its functions immediately. This should not change any reasonable existing code.
- Change how [`partition` `⊜`](https://uiua.org/docs/partition) and [`group` `⊕`](https://uiua.org/docs/group) work with dyadic functions to be consistent with [`reduce` `/`](https://uiua.org/docs/reduce)
- Deprecate [`restack` `⇵`](https://uiua.org/docs/restack). It was never a good idea.
- Remove the overloaded behavior of [`call` `!`](https://uiua.org/docs/call). It no longer behaves like an if-else when used with a list of functions.
  - You can replace all existing instances of that use case with `!⊡:`
- Add the [`if` `?`](https://uiua.org/docs/if) modifier, which calls one of two functions based on a condition

## 2023-10-02
### Language
- [`both` `∩`](https://uiua.org/docs/both) can now be used with a function that takes any number of arguments.
- Various bug and crash fixes
### Interpreter
- Tell the user when the interpreter can be updated

## 2023-10-01
- Add the [`dip` `⊙`](https://uiua.org/docs/dip) modifier, which temporarily pops a value
- Deprecate `roll↷` and `unroll↶`
- Add [`under` `⍜`](https://uiua.org/docs/under) [`keep` `▽`](https://uiua.org/docs/keep)
- Add [`dump`](https://uiua.org/docs/dump) function, which prints the entire stack

## 2023-09-30
### Language
- Remove the `|1.1` signature restriction for [`under` `⍜`](https://uiua.org/docs/under)'s second function
- Remove the rank`∴` function
- Remove the restriction that all functions in a non-scalar function array all have the compatible signatures
- Whether a binding is a constant or a function is now independent of how many values are on the stack
- Add a system for non-error diagnostics
  - Add advice about redundant uses of [`each` `∵`](https://uiua.org/docs/each)
### Interpreter
- Allow passing `--no-format` to `uiua watch`
- [`&sc`](https://uiua.org/docs/&sc) now returns `0` if EOF is input
### Website
- [`&sc`](https://uiua.org/docs/&sc) now works on the website by showing a prompt

## 2023-09-29
### Language
- Make binding names case-sensitive
- Add `^` syntax to terminate modifier parsing. There is a basic example [in the tutorial](http://uiua.org/docs/functions#terminating-modifiers).
- Add [`&runi`](https://uiua.org/docs/&runi) and [`&runc`](https://uiua.org/docs/&runc) functions for running commands
- Add [`&cd`](https://uiua.org/docs/&cd) function for changing the current working directory
- Add shadowable [constants](https://uiua.org/docs/constants) like `e` and `os`
- Change [`trident` `∋`](https://uiua.org/docs/trident) argument order to make it easier to reason about
- Enable [`fill` `⬚`](https://uiua.org/docs/fill) for [`keep` `▽`](https://uiua.org/docs/keep) if the amount list is shorter than the kept array
### Interpreter
- Add `uiua eval` command which evaluates a Uiua expression and prints the result
- Watch commands no longer try to open the file being watched
- Fix a bug that made numbers that were `≤` `1e-12` `⌵` format to `0`
### Website
- Make a space character `@ ` more visible by underlining the space
- Improve cursor movement when formatting in the editor

## 2023-09-28
### Language
- Add this changelog
- Add [`trace` `~`](https://uiua.org/docs/trace) function
  - Debug-prints the value on top of the stack without popping it
  - Shows the line and column number too
- Add [`both` `∩`](https://uiua.org/docs/both) modifier
  - This can change code like `/(|2 ⊂!∶!∶) {"a" "bc" "def"}`
  - To just `/'⊂∩! {"a" "bc" "def"}`
- Turn the term pair syntactic construct into a modifier called [`bind` `'`](https://uiua.org/docs/bind)
### Interpreter
- Fix some correctness bugs related to `under` and `invert`
- Fix a crash when trying to reverse an empty array
### Website
- Add a [right-to-left](https://uiua.org/rtl) explanation page
