if [ $1 ] && [ -r $1 ]
then
filename=${1/.snw/}
./target/release/winter "$filename.snw" &&
rustc "$filename.rs" -o "$filename" > /dev/null &&
rm -rf "$filename.rs" &&
chmod +x $filename &&
./$filename
else echo "Didn't Specify A Winter File To Compile"
fi
