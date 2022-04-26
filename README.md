# StegaCean
Steganography program written in pure rust. It changes the least important bit of every byte of an input .png file to the bits of the input text file. this was a experiment to learn about steganography and was made for educational purposes.


### Usage:

```
$ >  cargo build
# encoding
$ >  ./target/debug/StegaCean encode -m <message.txt> -p <picture.png> <output_picture.png>
# decodeing
$ >  ./target/debug/StegaCean decode <output_picture.png> <output_text_file.txt>
```





