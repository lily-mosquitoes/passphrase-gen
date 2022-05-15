# Passphrase Gen

Script for generating [xkcd-style](https://xkcd.com/936/) passphrases from a given word list (see [Diceware](https://theworld.com/~reinhold/diceware.html) for learning more about this).

## Usage

`./passphrase-gen --help` for help.

`./passphrase-gen --file word_list.csv` for polling 4 random words from the given `.csv` word list.

`./passphrase-gen --file word_list.csv --number_of_words 6` for polling 6 random words instead of the default (*n* = 4).

I like putting it in `.bash_aliases` like:
```bash
alias passphrase-gen='/path/to/binary -f /path/to/word_list.csv -n 6'
```
So I can type `passphrase-gen` from anywhere and get a list of words from my preferred list.

The expected format for the `.csv` word list is to have at least one header, called `word`, but it may have other 2 (optional) headers: `meaning` and `example_usage`; any other header will be ignored.

The words are selected using the [thread-local random number generator](https://docs.rs/rand/0.8.5/rand/rngs/struct.ThreadRng.html), which is seeded by the system. They are selected from the given `.csv` word list with a [uniform distribution](https://docs.rs/rand/0.8.5/rand/distributions/struct.Uniform.html).

A lower bound for the entropy is calculated by <span>log<sub>2</sub> ( <span>*p*<sup>*n*</sup><span>/</span>*n*!</span> )</span>, where *p* is the total number of words in the list and *n* is the number of words chosen. A lower bound is provided, in lieu of [simply taking the <span>log<sub>2</sub> ( *p*<sup>*n*</sup> )</span>](https://www.pleacher.com/mp/mlessons/algebra/entropy.html), to account for shifting of the words from any position to any position, with the purpose of constructing a meaningful sentence.

## Screenshots

![example image 1: words in Portuguese, meaning in English.](https://i.imgur.com/NhZkE90.png)

![example image 2: words in Japanese, meaning in English, example usage in Japanese.](https://i.imgur.com/6GWO5Tl.png)

## Building

Make sure you have [rust and cargo](https://www.rust-lang.org/tools/install) installed, then type from inside the project folder:

`cargo build --release`

The resulting binary should be in `./target/release/`.

## Copyright

This code is licensed under the GNU Affero General Public License version 3 or later. See [LICENSE](LICENSE) or https://www.gnu.org/licenses/agpl-3.0.en.html.
