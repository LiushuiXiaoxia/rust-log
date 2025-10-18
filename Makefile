
ANDROID_OUT=platform/android/nlog/src/main/jniLibs
IOS_OUT=platform/ios/nlog/
HARMONY_OUT=platform/harmony_demo/entry/libs/arm64-v8a

build:
	cargo clean && cargo build

androidLib: build
	rustup target add aarch64-linux-android armv7-linux-androideabi i686-linux-android x86_64-linux-android
	cargo install cargo-ndk
	cargo clean
	rm -rf $(ANDROID_OUT)
	RUSTFLAGS="-C link-arg=-Wl,-z,max-page-size=16384 -Wl,-z,common-page-size=16384" cargo ndk -t arm64-v8a -o $(ANDROID_OUT) build --release
	RUSTFLAGS="-C link-arg=-Wl,-z,max--page-size=16384 -Wl,-z,common-page-size=16384" cargo ndk -t armeabi-v7a -o $(ANDROID_OUT) build --release
	RUSTFLAGS="-C link-arg=-Wl,-z,max--page-size=16384 -Wl,-z,common-page-size=16384" cargo ndk -t x86_64 -o $(ANDROID_OUT) build --release
	RUSTFLAGS="-C link-arg=-Wl,-z,max--page-size=16384 -Wl,-z,common-page-size=16384" cargo ndk -t x86 -o $(ANDROID_OUT) build --release
	cp rustlog.h  $(ANDROID_OUT)

androidApp: androidLib
	cd platform/android && ./gradlew clean && ./gradlew assembleDebug


#iosLib: build
iosLib:
	rustup target add aarch64-apple-ios-sim
	cargo clean
	rm -rf $(IOS_OUT) && mkdir -p $(IOS_OUT)
	cargo build --target aarch64-apple-ios-sim --release && \
	cp target/aarch64-apple-ios-sim/release/librustlog.a $(IOS_OUT)
	cp rustlog.h  $(IOS_OUT)

harmonyLib: build
	rustup target add aarch64-unknown-linux-ohos
	cargo clean
	# rm -rf $(HARMONY_OUT)
	cargo build --target aarch64-unknown-linux-ohos --release
	cp target/aarch64-unknown-linux-ohos/release/librustlog.so  $(HARMONY_OUT)
	#cargo build --target aarch64-unknown-linux-ohos
	#cp target/aarch64-unknown-linux-ohos/debug/librustlog.so  $(HARMONY_OUT)
	cp rustlog.h  $(HARMONY_OUT)