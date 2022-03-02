# Downloads all inputs into the input/folder
# Use this file responsibly

if [ -z $1 ];
then
    echo "No year given... please enter a year!"
    exit
fi

if [ ! -f "cookie.txt" ]; then
    echo "Please create a cookie.txt file and enter your cookie in it.\nYou can find the cookie on the advent of code website using developer tools for your browser."
    exit
fi

if ! [ -x "$(command -v curl)" ];
then
    echo "To use this script you must have curl installed.\nPlease download and install curl: https://curl.se"
    exit
fi

cookie="$(<cookie.txt)"
END=25
for ((i=1;i<=END;i++)); do
    curl --cookie "session=$cookie" "https://adventofcode.com/$1/day/$i/input" > "day$i.txt"
done

