# wc

## wc の挙動確認

```sh
# 複数ファイルを指定
$ wc tests/inputs/*.txt
       4      29     177 tests/inputs/atlamal.txt
       0       0       0 tests/inputs/empty.txt
       1       9      48 tests/inputs/fox.txt
       5      38     225 total

# 複数ファイルを指定。存在しないファイルを指定。エラーメッセージは標準エラー出力に出力される。
$  wc tests/inputs/atlamal.txt tests/inputs/blargh tests/inputs/fox.txt
       4      29     177 tests/inputs/atlamal.txt
wc: tests/inputs/blargh: open: No such file or directory
       1       9      48 tests/inputs/fox.txt
       5      38     225 total

# 複数ファイルを指定。存在しないファイルを指定。エラーメッセージを/dev/null にリダイレクト。
$  wc tests/inputs/atlamal.txt tests/inputs/blargh tests/inputs/fox.txt 2>/dev/null
       4      29     177 tests/inputs/atlamal.txt
       1       9      48 tests/inputs/fox.txt
       5      38     225 total
```
