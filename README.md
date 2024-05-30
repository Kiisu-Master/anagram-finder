# Anagram finder

Place these files in your home dir:
```
https://gist.githubusercontent.com/MarvinJWendt/2f4f4154b8ae218600eb091a5706b5f4/raw/36b70dd6be330aa61cd4d4cdfda6234dcb0b8784/wordlist-german.txt,
https://github.com/dwyl/english-words/blob/master/words.txt
```
in your home dir and rename them `.anagram-dictionary-{de|en}.txt`
Then use `cargo run --release --quiet -- {de|en} {optional_word}`