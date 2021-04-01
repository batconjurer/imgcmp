# imgcmp
A library to compare two pictures using a perceptual hash

This repository is a submission to Mintlayer as part of their recruitment process. The challenge given is quoted here:

### Programming Task

Your goal is to create a tool that is able to compare 2 pictures and respond positively or negatively if the two pictures are the same.

From the command line it will work like that:
```
$ imgcmp picture1.jpg picture1_modified.jpg
Pictures are the same

$ imgcmp picture1.jpg different_picture.jpg
Pictures are different
```

The tool is supposed to be able to detect simple modifications of the pictures (like resizing, change of exposition, saturation, and maybe even small editing).

To help you with the development [here](https://github.com/erubboli/img_cmp) is a repository that cointains some example pictures and a `test.sh` file that **must** pass.

Note: the test is expecting a rust directory structure, feel free to adapt it to other languages.

#### Evaluation

It is OK to use external libraries, but the code should contain the algorithm responsible for the match, for example is OK to use [image](https://crates.io/crates/image). 

What is evaluated is the ability to understand an issue and implement an agorithm. It's up to you to decide the approach to use, the language, how to use tests and how to comment the code.


#### Hints
[Here](https://www.hackerfactor.com/blog/index.php?/archives/432-Looks-Like-It.html) are discussed two simple methods, **ahash** and **phash**, you don't have to follow it, the minimum requirement is to satisfy `test.sh`. 

---


