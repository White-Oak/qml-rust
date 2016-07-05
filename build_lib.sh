git clone https://github.com/filcuc/DOtherSide.git --branch add-support-for-qvariantlist --single-branch --depth 1
cd DOtherSide
rm -rf build
mkdir build
cd build

cmake ..
make DOtherSideStatic
