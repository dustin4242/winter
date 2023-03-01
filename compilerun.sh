if [ $1 ] && [ -r $1 ]
then
filename=${1/.snw/}
./target/release/winter "$filename.snw" &&
fasm "$filename.fasm" > /dev/null &&
rm -rf "$filename.fasm" &&
chmod +x $filename &&
./$filename
else echo "Didn't Specify A Winter File To Compile"
fi
