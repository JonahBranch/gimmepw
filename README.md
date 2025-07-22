# gimmepw - command line password generation utility

I am learning rust so I decided to whip up a command line utility I will find useful. I know there are lots of utilities for this but I couldn't find one in my package manager (Chimera linux btw) and the utilities I downloaded to compile myself had issues with the muslc system. There are other rust cli password generators which DO work on my machine perfectly fine, but for whatever reason I wasn't happy with their functionality (looking at you, passguard). I'm sure the perfect solution exists out there for me already, but this was a nice excuse to practice some rust.

This one is a little over 100 lines thanks to blarg, random-string, and rand doing all the heavy lifting. In fact, this is pretty much just a "random-string" cli wrapper with some extra logic to ensure we get the exact number of special characters and digits we want.

Design goals: 
- make a good password by default with no command line arguments
- NOT be interactive (looking at you passguard! No, I don't want to fill out your interactive questionnaire to generate a basic password)
- portability would be nice, but I haven't tested this on any other system. Please let me know if you have issues
- it should be secure. So far I'm just assuming that the random-string cargo package and the ```rand::rng()``` shuffle are good enough for this task but I'm no security researcher so let me know if this is incorrect

Here's the help page:
``` sh
> gimmepw -h
usage: organization [-h] [-c COLUMNS] [-d DIGITS] [-l LENGTH] [-r ROWS] [-s SPECIAL]

options:
 -h, --help                      Show this help message and exit.
 -c COLUMNS, --columns COLUMNS   Number of columns to generate
 -d DIGITS, --digits DIGITS      Number of digits each password should contain
 -l LENGTH, --length LENGTH      The length of the generated password(s)
 -r ROWS, --rows ROWS            Number of rows to generate
 -s SPECIAL, --special SPECIAL   Number of special characters each password
                                 should contain
```

and some example usage:
```
> gimmepw
h0g~dx4DbqKw
> gimmepw -r 5 -c 3
f2pz8eyeCp_X oPuO}wO09uPe Tjr@Wj90WINi
MyL!Y6NZmX4C %lNhJhYn1a8Y lR=SSmp0P6pW
JkR~HNJV1g2p 4o7R{QZfBDhg uF^xJlC5y8gP
DOZ~t0r5QJIC ck$ji8BaGob0 sH99cQsYkpn[
O7;NEqn3WIWx MYbf4Sax@X4q x6:JRalzB1Az
```

Possible improvements:
- gimmepw allows a LOT of special characters, the full list being ```~!@#$%^&*()-_=+{}[];:```. Some websites might not support all of these so it would be nice to exclude certain characters from passwords using another command line argument. It's possible I could remove the less common ones to improve the default behavior as well, and maybe add a feature to "add back" the characters you want available. Ideally the default behavior should make a valid password on as many websites as possible
- the best password generators are, in my opinion, pass phrases (at least if you intend on memorizing your passwords). Implementing this would be a little more complicated since you would need to include a dictionary. Maybe this would be a fun next step
