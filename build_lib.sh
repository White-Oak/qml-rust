git clone https://github.com/filcuc/DOtherSide.git --single-branch --depth 1
cd DOtherSide
rm -rf build
mkdir build
cd build

cmake ..

if [ "$IS_DYLIB" != "" ]; then
    make DOtherSide
else
    make DOtherSideStatic
fi
