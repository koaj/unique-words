This CLI app prints all unique words of a file in a simple way.

### Usage:
```bash
./uwords text.txt
```




#### Showcase:

```bash
▶ cat test.txt
one
two two
three Three
four, five, six
/home/address/public_dir
["Hello", "world!"]
root::user
<!doctype html><html><head><meta charset="utf-8"><title>



▶ ./uwords test.txt
one
two
three
Three
four
five
six
home
address
public_dir
Hello
world
root
user
doctype
html
head
meta
charset
utf-8
title
```
