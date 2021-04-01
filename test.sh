#!/bin/bash
BIN=target/release/imgcmp
OK="Pictures are the same"
KO="Pictures are different"

test_equal() {
        res=`$BIN $1 $2`
        if (test "$OK" = "$res"); 
        then 
                return 0
        else 
                echo "TEST FAILED"
                exit 1
        fi
}

test_different() {
        res=`$BIN $1 $2`
        if (test "$KO" = "$res"); 
        then 
                return 0
        else 
                echo "TEST FAILED"
                exit 1
        fi
}

test_equal ./assets/cat.jpg ./assets/cat_edited.jpg
test_equal ./assets/cat2.jpg ./assets/cat2_edited.jpg
test_equal ./assets/ferrari_roma.jpg ./assets/ferrari_roma_edited.png
test_equal ./assets/ferrari_roma2.jpg ./assets/ferrari_roma2_edited.jpg
test_different ./assets/cat2.jpg ./assets/cat_edited.jpg
test_different ./assets/ferrari_roma.jpg ./assets/cat_edited.jpg
test_different ./assets/cat.jpg ./assets/cat2_edited.jpg
test_different ./assets/ferrari_roma2.jpg ./assets/ferrari_roma_edited.png
test_different ./assets/ferrari_roma.jpg ./assets/ferrari_roma2_edited.jpg

echo "Congratulation, test passed!"
