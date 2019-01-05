#PREREQUISITES:
#android studio
#sdk 19
#make_standalone_toolchain.py --arch arm --api 19 --install-dir ./NDK

export ANDROID_HOME=~/Android/Sdk/
export NDK_HOME=$ANDROID_HOME/ndk-bundle/

cd ./android-project/app
~/Android/Sdk/ndk-bundle/ndk-build
#./
cd ../../
#copy libSDL2.so where cargo can see it
cp ./android-project/app/libs/armeabi-v7a/libSDL2.so ./NDK/arm-linux-androideabi/lib/armv7-a/libSDL2.so
cargo build --lib --target armv7-linux-androideabi
#copy libmain.so where gradlew can see it
cp target/armv7-linux-androideabi/debug/libsdl_main.so android-project/app/libs/armeabi-v7a/libmain.so
cd ./android-project
./gradlew "$@"