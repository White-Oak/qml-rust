git clone https://github.com/filcuc/DOtherSide.git --single-branch --depth 1
cd DOtherSide
rm -rf build
mkdir build
cd build

cmake ..
make DOtherSideStatic
