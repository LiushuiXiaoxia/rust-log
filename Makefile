
#ANDROID_OUT=out/jniLibs
ANDROID_OUT=platform/android/nlog/src/main/jniLibs

build:
	cargo clean && cargo build

android:
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
	cargo install cargo-ndk
	cargo clean
	rm -rf $(ANDROID_OUT)
	cargo ndk -t arm64-v8a -o $(ANDROID_OUT) build --release
	cargo ndk -t armeabi-v7a -o $(ANDROID_OUT) build --release
	cargo ndk -t x86_64 -o $(ANDROID_OUT) build --release
	cargo ndk -t x86 -o $(ANDROID_OUT) build --release
	cp rustlog.h  $(ANDROID_OUT)
