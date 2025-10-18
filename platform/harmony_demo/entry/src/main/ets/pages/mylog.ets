import fs from '@ohos.file.fs'
import { Context } from '@kit.AbilityKit';
// import { logInit, logInfo } from 'libentry.so'
import testNapi from 'libentry.so';

export enum LogLevel {
  DEBUG = 0,
  INFO,
  WARN,
  ERROR
}

export class Logger {
  private tag: string
  private logDir: string
  private level: LogLevel = LogLevel.DEBUG

  constructor(tag: string, context: Context) {
    this.tag = tag;
    this.logDir = context.filesDir + "/mylogs"
    if (!fs.accessSync(this.logDir)) {
      fs.mkdirSync(this.logDir)
    }
    console.log(`logDir = ${this.logDir}`)
    testNapi.logInit(this.logDir)
  }

  setLevel(level: LogLevel) {
    this.level = level
  }

  private format(level: string, msg: string) {
    const time = new Date().toISOString()
    return `[${time}] [${level}] [${this.tag}] ${msg}`
  }

  debug(msg: string) {
    if (this.level <= LogLevel.DEBUG) {
      console.debug(this.format("DEBUG", msg))
      testNapi.logMessage("DEBUG", this.tag, msg)
    }
  }

  info(msg: string) {
    if (this.level <= LogLevel.INFO) {
      console.info(this.format("INFO", msg))
      testNapi.logMessage("INFO", this.tag, msg)
    }
  }

  warn(msg: string) {
    if (this.level <= LogLevel.WARN) {
      console.warn(this.format("WARN", msg))
      testNapi.logMessage("WARN", this.tag, msg)
    }
  }

  error(msg: string) {
    if (this.level <= LogLevel.ERROR) {
      console.error(this.format("ERROR", msg))
      testNapi.logMessage("ERROR", this.tag, msg)
    }
  }
}


