import { hilog } from '@kit.PerformanceAnalysisKit';
import testNapi from 'libentry.so';
import { Logger } from './mylog'

const DOMAIN = 0x0000;

@Entry
@Component
struct Index {
  @State message: string = 'Hello World';
  init: boolean = false;
  logger: Logger | null = null
  private context = getContext()

  build() {
    Row() {
      Column() {
        Text(this.message)
          .fontSize($r('app.float.page_text_font_size'))
          .fontWeight(FontWeight.Bold)
          .onClick(() => {
            if (this.logger == null) {
              this.init = true;
              this.logger = new Logger("MyLog", this.context)
            }
            for (let index = 0; index < 10; index++) {
              this.logger?.debug(`test log from ets ${index}`)
              this.logger?.info(`test log from ets ${index}`)
              this.logger?.warn(`test log from ets ${index}`)
              this.logger?.error(`test log from ets ${index}`)
            }

            this.message = 'Welcome';
            hilog.info(DOMAIN, 'testTag', 'Test NAPI 2 + 3 = %{public}d', testNapi.add(2, 3));
          })
      }
      .width('100%')
    }
    .height('100%')
  }
}
