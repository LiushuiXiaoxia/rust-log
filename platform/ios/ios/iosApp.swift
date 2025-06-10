//
//  iosApp.swift
//  ios
//
//  Created by xiaqiulei on 2025-06-10.
//

import Foundation
import SwiftUI

@main
struct iosApp: App {

  init() {
    setup()
  }

  private func setup() {
    let fileManager = FileManager.default

    // 使用 Caches 目录作为下载目录（适合临时文件）
    guard let cachesURL = fileManager.urls(for: .cachesDirectory, in: .userDomainMask).first else {
      return
    }
    let downloadsURL = cachesURL.appendingPathComponent("logs")
    if !fileManager.fileExists(atPath: downloadsURL.path) {
      do {
        try fileManager.createDirectory(
          at: downloadsURL,
          withIntermediateDirectories: true,
          attributes: nil)

        print("downloadsURL = \(downloadsURL)")

      } catch {
        print("创建下载目录失败: \(error.localizedDescription)")
      }
    }
    log_init(nil, downloadsURL.path())
    print("downloadsURL = \(downloadsURL)")

    for i in 1...100 {
      log_write("debug", "ios", "test log msg \(i)")
      log_write("info", "ios", "test log msg \(i)")
      log_write("warn", "ios", "test log msg \(i)")
      log_write("error", "ios", "test log msg \(i)")
    }
  }

  var body: some Scene {
    WindowGroup {
      ContentView()
    }
  }
}
