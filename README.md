# StegaCean
Steganography program written in pure rust. It changes the least important bit of every byte of an input .png file to the bits of the input text file. this was a experiment to learn about steganography and was made for educational purposes. This technique works best with black and white pngs. the reason for this is that this strategy will only affect the last bit. meaning that the value of the individual RGB values for each pixel can change by a maximum of 1 per color. consequently, the largest shift from a pixel with RBG values [0, 0, 0] would be to [1, 1, 1]. this is not noticeable to the human eye. The same goes for pixels with RGB values [255, 255, 255].


### Usage:

```
$ >  cargo build
# encoding
$ >  ./target/debug/StegaCean encode -m <message.txt> -p <picture.png> <output_picture.png>
# decodeing
$ >  ./target/debug/StegaCean decode <output_picture.png> <output_text_file.txt>
```





