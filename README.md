# UULP

『Unix/Linuxプログラミング理論と実践』の読書記録

## Dockerコンテナの起動方法

```
$ docker build -t bionic-dev .
$ docker run --rm -it --cap-add=SYS_PTRACE --security-opt="seccomp=unconfined" --name bionic bionic-dev bash
```

### 注意

- `docker run`の引数は『Unix/Linuxプログラミング理論と実践』を読むための設定であり、実運用には一切向かない。
