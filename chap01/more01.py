import sys
import typing

PAGELEN = 24
LINELEN = 512


def main():
    if len(sys.argv[1:]) == 0:
        do_more(sys.stdin)
    else:
        for f in sys.argv[1:]:
            with open(f) as in_f:
                do_more(in_f)


def do_more(f: typing.TextIO):
    """
    PAGELEN行読み込み、ユーザーの入力を待つ。
    """

    num_of_lines: int = 0

    for line in f:
        if num_of_lines == PAGELEN:
            reply: int = see_more()
            if reply == 0:
                break
            num_of_lines -= reply
        print(line, end="")
        num_of_lines += 1


def see_more() -> int:
    """
    メッセージを出力して応答を待ち、入力値に応じて進める行数を返す。

    qならば終了、スペースならば次のページ、エンターならば次の行分進める。
    """
    print("\033[7m more? \033[m")
    c: str = input()
    if c == "q":
        return 0
    if c == " ":
        return PAGELEN
    if c == "":
        return 1
    return 0


if __name__ == "__main__":
    main()
