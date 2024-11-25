# Doubly Linked Listを実装しよう

[Rustを1ミリも知らない俺がRustプログラミングさせられた件](https://www.youtube.com/watch?v=6AiU6ncdUdk)
を参考にして、Doubly Linked Listを実装してみた

push_front, push_back, Displayは動画内で実装されていたので諸々追加した。

## pop_front, pop_back
DoublyLinkedListから要素の所有権を奪い、Optionで返す関数。
所有権を奪うために隣の要素からの参照だけでなく先頭と末尾への参照も消す必要があるところがポイント。

## From
スライスと配列からDoublyLinkedListを作成するためのFromトレイトを実装した。
Fromトレイトを実装するとinto()メソッドが使えるようになる。

# todo
## iter
iterを実装して非破壊で読み込みたいが、DoublyLinkedList内部の要素への参照を持つのは難しそう。
Rc<RefCell<T>>を使っているので外部に参照を持った要素があると構造が壊れそう。

## drop
相互参照を持つため、メモリリークが発生している。

# 追記
現状の実装ではiterの実装が難しい。
`read`にi番目の要素への参照を返す関数を実装しようとしたが、Rc<RefCell<T>>を使っているためできそうにない。
外部に参照`&`を持とうとすると、返却する要素のライフタイムがread内で終わってしまうため、コンパイルエラーになる。
`Ref`でも同様の問題が発生する。
ライフタイム変数を用意してDoublyLinkedListのライフタイムと一致させれば解決できそうだが、関数が所持する一時変数の返却はできない、というエラーになる。

また、Rc<RefCell<T>>で返却すると外部から要素を変更できてしまい、イミュータブルなのにミュータブルな操作ができてしまう。
pop_front, pop_backで要素を取り出すときに、自身への参照の個数が前後もしくは先頭末尾からの2つであることを前提としているため、外部から要素を変更できると破綻する。

この問題は[A Bad but Safe Doubly-Linked Deque](https://rust-unofficial.github.io/too-many-lists/)
にも書いてあるが、Rc<RefCell<T>>を使うという選択肢は間違っているということだろう。

次の章のunsafeを使って実装する、という方針がよさそう。

