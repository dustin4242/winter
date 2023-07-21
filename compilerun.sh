if [ $1 ] && [ -r $1 ]
then
filename=${1/.snw/}
echo "Building Compiler..."
cargo build --release > /dev/null &&
echo "Compiling..." &&
start=`date +%s%N | cut -b1-13`
./target/release/winter "$filename.snw" &&
rustc "$filename.rs" -o "$filename" -C opt-level=0 > /dev/null &&
end=`date +%s%N | cut -b1-13`
echo "Finished in $((end-start)) milliseconds"
rm -rf "$filename.rs" &&
chmod +x $filename &&
echo "Output:" && ./$filename
else echo "Didn't Specify A Winter File To Compile"
fi
